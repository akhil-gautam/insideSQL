use academy_core::{equivalent, Progress};

#[test]
fn identity_migration_preserves_history_without_rebinding_legacy_proof() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("progress.sqlite");
    let old = rusqlite::Connection::open(&path).unwrap();
    old.execute_batch("CREATE TABLE progress(exercise TEXT PRIMARY KEY,sql TEXT NOT NULL DEFAULT '',unlocked INTEGER NOT NULL DEFAULT 0,passed_revision TEXT); INSERT INTO progress VALUES('first','SELECT 1',1,'v1'); CREATE TABLE analysis(exercise TEXT PRIMARY KEY,revision TEXT NOT NULL,sql TEXT NOT NULL,plan TEXT NOT NULL);").unwrap();
    old.execute(
        "INSERT INTO analysis VALUES('first','v1','SELECT 1',?1)",
        [serde_json::json!([{"Plan":{"Actual Rows":1,"Actual Loops":1}}]).to_string()],
    )
    .unwrap();
    drop(old);
    for _ in 0..2 {
        let progress = Progress::open(&path).unwrap();
        assert_eq!(
            progress.script("first").unwrap().as_deref(),
            Some("SELECT 1")
        );
        assert!(progress.unlocked("first").unwrap() && progress.passed("first", "v1").unwrap());
        assert!(progress.has_analysis("first", "v1", "SELECT 1").unwrap());
        assert!(!progress
            .has_analysis_for_lab("first", "v1", "SELECT 1", "123:42")
            .unwrap());
    }
}

#[test]
fn identity_storage_matches_lab_and_does_not_accept_unbound_or_estimated_evidence() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("progress.sqlite");
    let progress = Progress::open(&path).unwrap();
    let measured = serde_json::json!([{"Plan":{"Actual Rows":1,"Actual Loops":1}}]);
    let estimate = serde_json::json!([{"Plan":{"Plan Rows":1}}]);
    assert!(progress
        .record_analysis_for_lab("first", "v1", "SELECT 1", &estimate, "123:42")
        .is_err());
    progress
        .record_analysis_for_lab("first", "v1", "SELECT 1", &measured, "123:42")
        .unwrap();
    assert!(progress
        .has_analysis_for_lab("first", "v1", "SELECT 1", "123:42")
        .unwrap());
    for (exercise, revision, sql, identity) in [
        ("first", "v1", "SELECT 1", "124:42"),
        ("first", "v1", "SELECT 1", "123:43"),
        ("first", "v2", "SELECT 1", "123:42"),
        ("first", "v1", "SELECT 2", "123:42"),
        ("other", "v1", "SELECT 1", "123:42"),
    ] {
        assert!(!progress
            .has_analysis_for_lab(exercise, revision, sql, identity)
            .unwrap());
    }
    drop(progress);
    let reopened = Progress::open(&path).unwrap();
    assert!(reopened
        .has_analysis_for_lab("first", "v1", "SELECT 1", "123:42")
        .unwrap());
    reopened
        .record_analysis("first", "v1", "SELECT 1", &measured)
        .unwrap();
    assert!(
        !reopened
            .has_analysis_for_lab("first", "v1", "SELECT 1", "123:42")
            .unwrap(),
        "A legacy upsert must not retain an old lab binding"
    );
}

#[test]
fn measured_storage_preserves_old_progress_and_rejects_estimates() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("progress.sqlite");
    let old = rusqlite::Connection::open(&path).unwrap();
    old.execute_batch("CREATE TABLE progress(exercise TEXT PRIMARY KEY,sql TEXT NOT NULL DEFAULT '',unlocked INTEGER NOT NULL DEFAULT 0,passed_revision TEXT); INSERT INTO progress VALUES('first','SELECT 1',1,'v1');").unwrap();
    drop(old);
    let progress = Progress::open(&path).unwrap();
    assert!(progress.passed("first", "v1").unwrap() && progress.unlocked("first").unwrap());
    assert_eq!(
        progress.script("first").unwrap().as_deref(),
        Some("SELECT 1")
    );
    assert!(!progress.has_analysis("first", "v1", "SELECT 1").unwrap());
    let estimate = serde_json::json!([{"Plan":{"Plan Rows":1}}]);
    assert!(progress
        .record_analysis("first", "v1", "SELECT 1", &estimate)
        .is_err());
    let measured = serde_json::json!([{"Plan":{"Actual Rows":1,"Actual Loops":1}}]);
    progress
        .record_analysis("first", "v1", "SELECT 1", &measured)
        .unwrap();
    assert!(!progress.has_analysis("other", "v1", "SELECT 1").unwrap());
    assert!(!progress.has_analysis("first", "v2", "SELECT 1").unwrap());
    assert!(!progress.has_analysis("first", "v1", "SELECT 2").unwrap());
    assert!(Progress::open(&path)
        .unwrap()
        .has_analysis("first", "v1", "SELECT 1")
        .unwrap());
}

#[test]
fn saving_a_draft_preserves_it_without_unlocking_or_changing_a_pass() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("progress.sqlite");
    let progress = Progress::open(&path).unwrap();
    progress.save_script("first", "SELECT id FROM").unwrap();
    let reopened = Progress::open(&path).unwrap();
    assert_eq!(
        reopened.script("first").unwrap().as_deref(),
        Some("SELECT id FROM")
    );
    assert!(!reopened.unlocked("first").unwrap());
    progress.attempt("first", "SELECT 1", "").unwrap();
    progress.pass("first", "v1").unwrap();
    progress.save_script("first", "-- next experiment").unwrap();
    assert!(reopened.passed("first", "v1").unwrap());
    assert!(reopened.unlocked("first").unwrap());
}

#[test]
fn syntax_error_attempt_unlocks_and_survives_reopening() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join("progress.sqlite");
    let progress = Progress::open(&path).unwrap();
    assert!(!progress.unlocked("first").unwrap());
    assert!(progress.attempt("first", "SELEC", "SELECT ...").unwrap());
    assert!(Progress::open(&path).unwrap().unlocked("first").unwrap());
}

#[test]
fn comments_and_unchanged_starters_do_not_unlock() {
    let directory = tempfile::tempdir().unwrap();
    let progress = Progress::open(&directory.path().join("progress.sqlite")).unwrap();
    for sql in [
        "",
        "   ",
        "-- SELECT 1",
        "/* outer /* nested */ comment */",
        " SELECT ... ",
    ] {
        assert!(!progress.attempt("first", sql, "SELECT ...").unwrap());
        assert!(!progress.unlocked("first").unwrap());
    }
}

#[test]
fn changed_grader_requires_rechecking_but_keeps_answer_unlocked() {
    let directory = tempfile::tempdir().unwrap();
    let progress = Progress::open(&directory.path().join("progress.sqlite")).unwrap();
    progress.attempt("first", "SELECT 1", "").unwrap();
    progress.pass("first", "v1").unwrap();
    assert!(progress.passed("first", "v1").unwrap());
    assert!(!progress.passed("first", "v2").unwrap());
    assert!(progress.unlocked("first").unwrap());
}

#[test]
fn result_comparison_preserves_duplicates_nulls_and_optional_order() {
    let row = |value: Option<&str>| vec![value.map(str::to_owned)];
    let actual = vec![row(Some("a")), row(None), row(Some("a"))];
    assert!(equivalent(
        &actual,
        &[row(None), row(Some("a")), row(Some("a"))],
        false
    ));
    assert!(!equivalent(&actual, &[row(None), row(Some("a"))], false));
    assert!(!equivalent(
        &actual,
        &[row(None), row(Some("a")), row(Some("a"))],
        true
    ));
    assert!(!equivalent(&[row(None)], &[row(Some("NULL"))], false));
}
