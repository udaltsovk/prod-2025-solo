use lazy_static::lazy_static;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref CHAR_REPLACEMENTS: HashMap<&'static str, Vec<&'static str>> = create_char_map();
    static ref TRANSLIT_MAP: HashMap<&'static str, &'static str> = create_translit_map();
    static ref WORD_REGEX: Regex = Regex::new(r"(?i)\b[\w']+\b").unwrap();
}

pub fn extract_words(text: &str) -> Vec<&str> {
    WORD_REGEX.find_iter(text).map(|m| m.as_str()).collect()
}

pub fn process_words(words: &[&str]) -> Vec<String> {
    words
        .par_iter()
        .flat_map(|word| normalize_word(word))
        .collect()
}

fn create_char_map() -> HashMap<&'static str, Vec<&'static str>> {
    let mut m = HashMap::new();
    m.insert("0", vec!["о", "о"]);
    m.insert("1", vec!["і", "l", "ӏ", "|"]);
    m.insert("3", vec!["е", "з", "э", "€"]);
    m.insert("4", vec!["ч", "ҷ", "ћ"]);
    m.insert("5", vec!["ѕ", "s", "$"]);
    m.insert("6", vec!["б", "b"]);
    m.insert("7", vec!["т", "t"]);
    m.insert("8", vec!["в", "ъ"]);
    m.insert("9", vec!["д", "g"]);
    m.insert("@", vec!["а", "a"]);
    m.insert("$", vec!["ѕ", "s"]);
    m.insert("!", vec!["і", "i"]);
    m.insert("*", vec![]);
    m.insert("#", vec![]);
    m.insert("ё", vec!["е"]);
    m.insert("ў", vec!["у"]);
    m.insert("ї", vec!["i"]);

    m.insert("a", vec!["а", "@"]);
    m.insert("b", vec!["б", "6"]);
    m.insert("c", vec!["ц", "с"]);
    m.insert("d", vec!["д"]);
    m.insert("e", vec!["е", "ё"]);
    m.insert("f", vec!["ф"]);
    m.insert("g", vec!["г", "9"]);
    m.insert("h", vec!["х"]);
    m.insert("i", vec!["і", "1", "!"]);
    m.insert("j", vec!["й"]);
    m.insert("k", vec!["к"]);
    m.insert("l", vec!["л"]);
    m.insert("m", vec!["м"]);
    m.insert("n", vec!["н"]);
    m.insert("o", vec!["о", "0"]);
    m.insert("p", vec!["п", "р"]);
    m.insert("q", vec!["к"]);
    m.insert("r", vec!["р"]);
    m.insert("s", vec!["с", "5"]);
    m.insert("t", vec!["т", "7"]);
    m.insert("u", vec!["у"]);
    m.insert("v", vec!["в", "8"]);
    m.insert("w", vec!["в"]);
    m.insert("x", vec!["кс", "х"]);
    m.insert("y", vec!["ы", "у"]);
    m.insert("z", vec!["з"]);
    m
}

fn create_translit_map() -> HashMap<&'static str, &'static str> {
    let mut m = HashMap::new();
    m.insert("a", "а");
    m.insert("b", "б");
    m.insert("c", "ц");
    m.insert("d", "д");
    m.insert("e", "е");
    m.insert("f", "ф");
    m.insert("g", "г");
    m.insert("h", "х");
    m.insert("i", "и");
    m.insert("j", "й");
    m.insert("k", "к");
    m.insert("l", "л");
    m.insert("m", "м");
    m.insert("n", "н");
    m.insert("o", "о");
    m.insert("p", "п");
    m.insert("q", "к");
    m.insert("r", "р");
    m.insert("s", "с");
    m.insert("t", "т");
    m.insert("u", "у");
    m.insert("v", "в");
    m.insert("w", "в");
    m.insert("x", "кс");
    m.insert("y", "ы");
    m.insert("z", "з");
    m.insert("sh", "ш");
    m.insert("shch", "щ");
    m.insert("ch", "ч");
    m.insert("zh", "ж");
    m.insert("yu", "ю");
    m.insert("ya", "я");
    m
}

pub fn normalize_word(word: &str) -> Vec<String> {
    let transliterated = transliterate(word);
    let variants = generate_variants(&transliterated);
    variants
        .into_iter()
        .map(|v| v.to_lowercase())
        .filter(|s| !s.is_empty())
        .collect()
}

fn transliterate(word: &str) -> String {
    let mut result = String::new();
    let mut chars = word.chars().peekable();

    while let Some(c) = chars.next() {
        let mut found = false;
        for (k, v) in TRANSLIT_MAP.iter() {
            if c.to_string().as_str() == *k {
                result.push_str(v);
                found = true;
                break;
            }
        }
        if !found {
            result.push(c);
        }
    }
    result
}

fn generate_variants(word: &str) -> Vec<String> {
    let mut variants = vec![String::new()];

    for c in word.chars() {
        let replacements = CHAR_REPLACEMENTS
            .get(c.to_string().as_str())
            .map(|v| {
                if v.is_empty() {
                    vec![c.to_string()]
                } else {
                    v.into_iter().map(|s| s.to_string()).collect()
                }
            })
            .unwrap_or_else(|| vec![c.to_string()]);

        variants = replacements
            .into_iter()
            .flat_map(|r| variants.iter().map(move |v| format!("{}{}", v, r)))
            .collect();
    }

    variants
}
