use flate2::read::GzDecoder;
use phf_codegen::Set;
use std::{
    collections::HashSet,
    env,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

fn main() {
    let dict_dir = Path::new("dictionaries");

    let mut unique_words = HashSet::new();
    let mut phf_set = Set::new();

    process_directory(dict_dir, &mut phf_set, &mut unique_words);

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("dictionary.rs");

    fs::write(
        &dest_path,
        format!(
            "static BAD_WORDS: phf::Set<&'static str> = {};\n",
            phf_set.build()
        ),
    )
    .unwrap();
}

fn process_directory(path: &Path, phf_set: &mut Set<String>, unique_words: &mut HashSet<String>) {
    let entries = fs::read_dir(path)
        .unwrap_or_else(|_| panic!("Failed to read directory: {}", path.display()));

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            process_directory(&path, phf_set, unique_words);
        } else {
            process_file(&path, phf_set, unique_words);
        }
    }
}

fn process_file(path: &PathBuf, phf_set: &mut Set<String>, unique_words: &mut HashSet<String>) {
    match path.extension().and_then(|s| s.to_str()) {
        Some("txt") => load_text_file(path, phf_set, unique_words),
        Some("gz") => load_gzip_file(path, phf_set, unique_words),
        _ => {}
    }
}

fn load_text_file(path: &PathBuf, phf_set: &mut Set<String>, unique_words: &mut HashSet<String>) {
    let file =
        File::open(path).unwrap_or_else(|_| panic!("Failed to open file: {}", path.display()));

    for line in BufReader::new(file).lines() {
        process_line(line.unwrap().as_str(), phf_set, unique_words);
    }
}

fn load_gzip_file(path: &PathBuf, phf_set: &mut Set<String>, unique_words: &mut HashSet<String>) {
    let file =
        File::open(path).unwrap_or_else(|_| panic!("Failed to open gzip file: {}", path.display()));

    let decoder = GzDecoder::new(file);
    for line in BufReader::new(decoder).lines() {
        process_line(line.unwrap().as_str(), phf_set, unique_words);
    }
}

fn process_line(line: &str, phf_set: &mut Set<String>, unique_words: &mut HashSet<String>) {
    let word = line.trim().to_lowercase();
    if !(word.is_empty() || word.starts_with('#') || unique_words.contains(&word)) {
        unique_words.insert(word.clone());
        phf_set.entry(word);
    }
}
