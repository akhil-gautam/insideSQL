#[allow(dead_code)]
#[path = "../build.rs"]
mod build_script;

use std::{fs, path::Path};

fn chapter(directory: &Path, name: &str) {
    fs::write(directory.join(name), "{}").unwrap();
}

#[test]
fn discovers_only_contiguous_authored_chapters_in_numeric_order() {
    let course = tempfile::tempdir().unwrap();
    chapter(course.path(), "chapter02.json");
    chapter(course.path(), "chapter01.json");
    chapter(course.path(), "notes.json");
    fs::create_dir(course.path().join("fixtures")).unwrap();
    let paths = build_script::discover_chapters(course.path()).unwrap();
    assert_eq!(
        paths
            .iter()
            .map(|p| p.file_name().unwrap().to_str().unwrap())
            .collect::<Vec<_>>(),
        ["chapter01.json", "chapter02.json"]
    );
    chapter(course.path(), "chapter04.json");
    assert!(build_script::discover_chapters(course.path()).is_err());
}

#[test]
fn rejects_empty_malformed_out_of_range_and_non_file_sources() {
    for name in [
        "chapter1.json",
        "chapter00.json",
        "chapter85.json",
        "chapterAA.json",
    ] {
        let course = tempfile::tempdir().unwrap();
        chapter(course.path(), "chapter01.json");
        chapter(course.path(), name);
        assert!(
            build_script::discover_chapters(course.path()).is_err(),
            "{name}"
        );
    }
    let course = tempfile::tempdir().unwrap();
    assert!(build_script::discover_chapters(course.path()).is_err());
    fs::create_dir(course.path().join("chapter01.json")).unwrap();
    assert!(build_script::discover_chapters(course.path()).is_err());
}

#[cfg(unix)]
#[test]
fn rejects_symlinked_chapter_sources() {
    let course = tempfile::tempdir().unwrap();
    chapter(course.path(), "notes.json");
    std::os::unix::fs::symlink(
        course.path().join("notes.json"),
        course.path().join("chapter01.json"),
    )
    .unwrap();
    assert!(build_script::discover_chapters(course.path()).is_err());
}
