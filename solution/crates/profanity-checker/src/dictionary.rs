use std::usize;

use levenshtein::levenshtein;

include!(concat!(env!("OUT_DIR"), "/dictionary.rs"));

fn check_typos(
    word: &str,
    typo_check: bool,
    min_length_for_typo: usize,
    max_typo_distance: usize,
) -> bool {
    if !typo_check || word.len() < min_length_for_typo {
        return false;
    }

    BAD_WORDS.iter().any(|bad_word| {
        let verdict = bad_word.len() >= min_length_for_typo
            && levenshtein(bad_word, word) <= max_typo_distance;
        if verdict {
            dbg!(word, bad_word);
        }
        verdict
    })
}

pub fn is_profane(
    word: &str,
    typo_check: bool,
    min_length_for_typo: usize,
    max_typo_distance: usize,
) -> bool {
    BAD_WORDS.contains(word)
        || check_typos(word, typo_check, min_length_for_typo, max_typo_distance)
}
