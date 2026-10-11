use academy_core::{equivalent, Progress};

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
