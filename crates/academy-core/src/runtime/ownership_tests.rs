use super::*;

fn runtime() -> PathBuf {
    std::env::var_os("ACADEMY_POSTGRES")
        .map(PathBuf::from)
        .unwrap_or_else(|| "/Applications/Postgres.app/Contents/Versions/18".into())
}

#[tokio::test]
async fn ownership_duplicate_start_preserves_the_running_owner() {
    let home = tempfile::tempdir().unwrap();
    let first = Cluster::start(home.path(), &runtime()).await.unwrap();
    first
        .create_lab("lab_01_01", "student_01_01", Fixture::Orders, 0)
        .await
        .unwrap();
    let admin = first.connect("lab_01_01", "academy_admin").await.unwrap();
    admin
        .batch_execute("UPDATE orders SET total=999.99 WHERE id=1")
        .await
        .unwrap();
    let learner = first.connect("lab_01_01", "student_01_01").await.unwrap();
    let credentials = fs::read(home.path().join("credentials")).unwrap();
    let pid = fs::read(home.path().join("data/postmaster.pid")).unwrap();
    let log = fs::read(home.path().join("postgres.log")).unwrap();
    let rejected = Cluster::start(home.path(), &runtime()).await.err().unwrap();
    let result = learner
        .query_one("SELECT total::text FROM orders WHERE id=1", &[])
        .await;
    assert!(
        result.is_ok(),
        "Rejected startup stopped the running owner: {rejected}; query: {}",
        result
            .as_ref()
            .err()
            .map(ToString::to_string)
            .unwrap_or_default()
    );
    assert_eq!(result.unwrap().get::<_, String>(0), "999.99");
    let other_process = Command::new(std::env::current_exe().unwrap())
        .args([
            "--ignored",
            "--exact",
            "runtime::ownership_tests::ownership_crash_worker",
            "--test-threads=1",
        ])
        .env("ACADEMY_OWNERSHIP_CRASH_HOME", home.path())
        .output()
        .unwrap();
    assert!(
        !other_process.status.success() && other_process.status.code() != Some(86),
        "A second process acquired the same workspace"
    );
    assert!(
        String::from_utf8_lossy(&other_process.stdout).contains("already open"),
        "The competing process must fail at workspace ownership, not unrelated setup"
    );
    assert_eq!(
        learner
            .query_one("SELECT total::text FROM orders WHERE id=1", &[])
            .await
            .unwrap()
            .get::<_, String>(0),
        "999.99"
    );
    assert_eq!(
        fs::read(home.path().join("data/postmaster.pid")).unwrap(),
        pid
    );
    assert_eq!(
        fs::read(home.path().join("credentials")).unwrap(),
        credentials
    );
    assert!(
        fs::read(home.path().join("postgres.log"))
            .unwrap()
            .starts_with(&log),
        "Rejected startup truncated the owner's log"
    );
    first.stop().unwrap();
}

#[tokio::test]
#[ignore = "explicitly invoked by ownership_crashed_app_does_not_authorize_stopping_its_live_server"]
async fn ownership_crash_worker() {
    let Some(home) = std::env::var_os("ACADEMY_OWNERSHIP_CRASH_HOME") else {
        return;
    };
    let cluster = Cluster::start(Path::new(&home), &runtime()).await.unwrap();
    cluster
        .create_lab("lab_01_01", "student_01_01", Fixture::Orders, 0)
        .await
        .unwrap();
    let admin = cluster.connect("lab_01_01", "academy_admin").await.unwrap();
    admin
        .batch_execute("UPDATE orders SET total=999.99 WHERE id=1")
        .await
        .unwrap();
    // Exit bypasses Drop while leaving the explicitly test-owned PostgreSQL child alive.
    std::process::exit(86);
}

#[tokio::test]
async fn ownership_crashed_app_does_not_authorize_stopping_its_live_server() {
    let home = tempfile::tempdir().unwrap();
    let child = Command::new(std::env::current_exe().unwrap())
        .args([
            "--ignored",
            "--exact",
            "runtime::ownership_tests::ownership_crash_worker",
            "--test-threads=1",
        ])
        .env("ACADEMY_OWNERSHIP_CRASH_HOME", home.path())
        .output()
        .unwrap();
    assert_eq!(
        child.status.code(),
        Some(86),
        "{}",
        String::from_utf8_lossy(&child.stderr)
    );
    let pid = fs::read_to_string(home.path().join("data/postmaster.pid")).unwrap();
    assert_eq!(
        pid.lines().nth(1).unwrap(),
        home.path()
            .join("data")
            .canonicalize()
            .unwrap()
            .to_str()
            .unwrap()
    );
    let log = fs::read(home.path().join("postgres.log")).unwrap();
    let credentials = fs::read(home.path().join("credentials")).unwrap();
    let error = Cluster::start(home.path(), &runtime()).await.err().unwrap();
    let still_running = Command::new(runtime().join("bin/pg_ctl"))
        .args(["status", "-D"])
        .arg(home.path().join("data"))
        .output()
        .unwrap();
    // Capture evidence and stop only this validated test-owned server before asserting.
    let same_pid = fs::read_to_string(home.path().join("data/postmaster.pid"))
        .ok()
        .as_deref()
        == Some(pid.as_str());
    let same_log = fs::read(home.path().join("postgres.log"))
        .unwrap()
        .starts_with(&log);
    if still_running.status.success() {
        assert!(Command::new(runtime().join("bin/pg_ctl"))
            .args(["stop", "-D"])
            .arg(home.path().join("data"))
            .args(["-m", "immediate", "-w", "-t", "5"])
            .status()
            .unwrap()
            .success());
    }
    assert!(
        still_running.status.success() && same_pid && same_log,
        "Rejected orphan startup disturbed its existing server: {error}"
    );
    assert!(error.contains("postmaster.pid"), "Process exit must release the app lock, leaving the server-marker guard responsible: {error}");
    assert_eq!(
        fs::read(home.path().join("credentials")).unwrap(),
        credentials
    );
    let resumed = Cluster::start(home.path(), &runtime()).await.unwrap();
    assert_eq!(
        resumed
            .connect("lab_01_01", "student_01_01")
            .await
            .unwrap()
            .query_one("SELECT total::text FROM orders WHERE id=1", &[])
            .await
            .unwrap()
            .get::<_, String>(0),
        "999.99"
    );
    resumed.stop().unwrap();
}

#[tokio::test]
async fn ownership_stop_releases_before_old_drop_and_independent_directories_coexist() {
    let home = tempfile::tempdir().unwrap();
    let other_home = tempfile::tempdir().unwrap();
    let first = Cluster::start(home.path(), &runtime()).await.unwrap();
    let independent = Cluster::start(other_home.path(), &runtime()).await.unwrap();
    assert_ne!(first.port, independent.port);
    first.stop().unwrap();
    let successor = Cluster::start(home.path(), &runtime()).await.unwrap();
    drop(first);
    for cluster in [&successor, &independent] {
        assert_eq!(
            cluster
                .connect("postgres", "academy_admin")
                .await
                .unwrap()
                .query_one("SELECT 42", &[])
                .await
                .unwrap()
                .get::<_, i32>(0),
            42
        );
        cluster.stop().unwrap();
    }
}

#[tokio::test]
async fn ownership_dead_child_cleanup_does_not_signal_a_different_server() {
    let owner_home = tempfile::tempdir().unwrap();
    let exited_home = tempfile::tempdir().unwrap();
    let owner = Cluster::start(owner_home.path(), &runtime()).await.unwrap();
    let exited = Cluster::start(exited_home.path(), &runtime())
        .await
        .unwrap();
    let connection = owner.connect("postgres", "academy_admin").await.unwrap();
    assert!(Command::new(runtime().join("bin/pg_ctl"))
        .args(["stop", "-D"])
        .arg(exited_home.path().join("data"))
        .args(["-m", "fast", "-w", "-t", "5"])
        .status()
        .unwrap()
        .success());
    // Replace only test-owned metadata after the owned child exits; it is not shutdown authority.
    let other_pid = fs::read(owner_home.path().join("data/postmaster.pid")).unwrap();
    fs::write(exited_home.path().join("data/postmaster.pid"), &other_pid).unwrap();
    let cleanup = exited.stop();
    let unaffected = connection.query_one("SELECT 42", &[]).await;
    assert!(
        unaffected.is_ok(),
        "Cleanup of an exited child stopped a different test-owned server; cleanup error: {}",
        cleanup.as_ref().err().cloned().unwrap_or_default()
    );
    assert_eq!(unaffected.unwrap().get::<_, i32>(0), 42);
    assert!(
        cleanup.is_ok(),
        "An already-exited owned child needs no server signal"
    );
    assert_eq!(
        fs::read(exited_home.path().join("data/postmaster.pid")).unwrap(),
        other_pid
    );
    fs::remove_file(exited_home.path().join("data/postmaster.pid")).unwrap();
    let retry = Cluster::start(exited_home.path(), &runtime())
        .await
        .unwrap();
    retry.stop().unwrap();
    owner.stop().unwrap();
}

#[cfg(unix)]
#[tokio::test]
async fn ownership_canonical_alias_cannot_bypass_the_owner() {
    let home = tempfile::tempdir().unwrap();
    let alias_home = tempfile::tempdir().unwrap();
    let first = Cluster::start(home.path(), &runtime()).await.unwrap();
    let alias = alias_home.path().join("alias");
    std::os::unix::fs::symlink(home.path(), &alias).unwrap();
    assert!(Cluster::start(&alias, &runtime()).await.is_err());
    assert_eq!(
        first
            .connect("postgres", "academy_admin")
            .await
            .unwrap()
            .query_one("SELECT 42", &[])
            .await
            .unwrap()
            .get::<_, i32>(0),
        42
    );
    first.stop().unwrap();
}

#[tokio::test]
async fn ownership_live_child_shutdown_ignores_replaced_directory_pid() {
    let owner_home = tempfile::tempdir().unwrap();
    let replaced_home = tempfile::tempdir().unwrap();
    let owner = Cluster::start(owner_home.path(), &runtime()).await.unwrap();
    let replaced = Cluster::start(replaced_home.path(), &runtime())
        .await
        .unwrap();
    let connection = owner.connect("postgres", "academy_admin").await.unwrap();
    let actual_pid = fs::read(replaced_home.path().join("data/postmaster.pid")).unwrap();
    let other_pid = fs::read(owner_home.path().join("data/postmaster.pid")).unwrap();
    fs::write(replaced_home.path().join("data/postmaster.pid"), &other_pid).unwrap();
    let stopped = replaced.stop();
    let untouched = connection.query_one("SELECT 42", &[]).await;
    // Only test-owned bytes are restored for diagnostics/cleanup if shutdown fails.
    if !stopped.is_ok() {
        fs::write(
            replaced_home.path().join("data/postmaster.pid"),
            &actual_pid,
        )
        .unwrap();
    }
    assert!(
        untouched.is_ok(),
        "Live-child shutdown signalled an unrelated test-owned directory PID"
    );
    assert!(
        stopped.is_ok(),
        "The actual child, not directory metadata, must determine shutdown"
    );
    assert_eq!(
        fs::read(owner_home.path().join("data/postmaster.pid")).unwrap(),
        other_pid
    );
    let reopened = Cluster::start(replaced_home.path(), &runtime())
        .await
        .unwrap();
    reopened.stop().unwrap();
    owner.stop().unwrap();
}

#[tokio::test]
async fn ownership_markers_and_unusable_lock_paths_fail_without_mutating_shared_state() {
    for marker in ["123456\n/unrelated\n", "malformed\n", ""] {
        let home = tempfile::tempdir().unwrap();
        fs::create_dir(home.path().join("data")).unwrap();
        fs::write(home.path().join("data/postmaster.pid"), marker).unwrap();
        let error = Cluster::start(home.path(), &runtime()).await.err().unwrap();
        assert!(error.contains("postmaster.pid"), "{error}");
        assert_eq!(
            fs::read_to_string(home.path().join("data/postmaster.pid")).unwrap(),
            marker
        );
        for file in ["credentials", "postgres.log", "data/PG_VERSION"] {
            assert!(!home.path().join(file).exists());
        }
        // Remove only this test's synthetic marker to verify failed-start lock release.
        fs::remove_file(home.path().join("data/postmaster.pid")).unwrap();
        let retry = Cluster::start(home.path(), &runtime()).await.unwrap();
        retry.stop().unwrap();
    }
    let home = tempfile::tempdir().unwrap();
    fs::create_dir(home.path().join(".academy.lock")).unwrap();
    assert!(Cluster::start(home.path(), &runtime()).await.is_err());
    for file in ["credentials", "postgres.log", "data"] {
        assert!(!home.path().join(file).exists());
    }
    fs::remove_dir(home.path().join(".academy.lock")).unwrap();
    let retry = Cluster::start(home.path(), &runtime()).await.unwrap();
    retry.stop().unwrap();
}
