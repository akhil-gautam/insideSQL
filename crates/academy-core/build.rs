use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn discover_chapters(course: &Path) -> Result<Vec<PathBuf>, String> {
    let mut chapters = Vec::new();
    for entry in fs::read_dir(course).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if !name.starts_with("chapter") || !name.ends_with(".json") {
            continue;
        }
        let bytes = name.as_bytes();
        if bytes.len() != 14
            || !bytes[7..9].iter().all(u8::is_ascii_digit)
            || !entry.file_type().map_err(|e| e.to_string())?.is_file()
        {
            return Err(format!("Invalid authored chapter source: {name}"));
        }
        let number: usize = name[7..9].parse().map_err(|e| format!("{e}"))?;
        if !(1..=84).contains(&number) {
            return Err(format!("Chapter number outside 01–84: {name}"));
        }
        chapters.push(entry.path());
    }
    chapters.sort();
    if chapters.is_empty() {
        return Err("No authored chapters found".into());
    }
    for (index, path) in chapters.iter().enumerate() {
        if path.file_name().unwrap() != format!("chapter{:02}.json", index + 1).as_str() {
            return Err(format!("Missing authored chapter {:02}", index + 1));
        }
    }
    Ok(chapters)
}

fn main() {
    let manifest = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let course = manifest.parent().unwrap().parent().unwrap().join("course");
    println!("cargo:rerun-if-changed={}", course.display());
    let chapters = discover_chapters(&course).expect("Invalid course catalog");
    let mut embedded = String::from("&[\n");
    for path in chapters {
        embedded.push_str(&format!("include_str!({:?}),\n", path));
    }
    embedded.push_str("]");
    let output = PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("chapters.rs");
    fs::write(output, embedded).expect("Cannot embed course catalog");
}
