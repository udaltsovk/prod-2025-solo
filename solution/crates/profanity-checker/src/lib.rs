mod context;
mod dictionary;
mod normalization;

pub use context::ContextAnalysis;
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;

const MAX_LEVENSHTEIN_DISTANCE: usize = 1;
const MIN_WORD_LENGTH_FOR_TYPO: usize = 4;

pub struct ProfanityChecker {
    context_analysis: bool,
    typo_check: bool,
    max_typo_distance: usize,
    min_length_for_typo: usize,
}

impl Default for ProfanityChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl ProfanityChecker {
    pub fn new() -> Self {
        ProfanityChecker {
            context_analysis: false,
            typo_check: false,
            max_typo_distance: MAX_LEVENSHTEIN_DISTANCE,
            min_length_for_typo: MIN_WORD_LENGTH_FOR_TYPO,
        }
    }

    pub fn with_typo_check(mut self, max_distance: usize, min_length: usize) -> Self {
        self.typo_check = true;
        self.max_typo_distance = max_distance;
        self.min_length_for_typo = min_length;
        self
    }

    pub fn with_context_analysis(mut self, enable: bool) -> Self {
        self.context_analysis = enable;
        self
    }

    pub fn check(&self, text: &str) -> bool {
        self.check_with_details(text).0
    }

    pub fn check_with_details(&self, text: &str) -> (bool, Vec<String>, f32) {
        let words = normalization::extract_words(text);
        let normalized_words = normalization::process_words(&words);

        let (matches, has_profanity) = self.check_profanity(&normalized_words);
        let context_score = if self.context_analysis {
            context::analyze_context(&words)
        } else {
            0.0
        };

        (
            has_profanity || context_score >= 0.7,
            matches,
            context_score,
        )
    }
    fn check_profanity(&self, words: &[String]) -> (Vec<String>, bool) {
        let has_profanity = AtomicBool::new(false);
        let matches = Mutex::new(Vec::new());

        words.par_iter().for_each(|word| {
            if dictionary::is_profane(
                word,
                self.typo_check,
                self.min_length_for_typo,
                self.max_typo_distance,
            ) {
                has_profanity.store(true, Ordering::Relaxed);
                matches.lock().unwrap().push(word.clone());
            }
        });

        let matches = matches.into_inner().unwrap();
        let has_profanity = has_profanity.load(Ordering::Relaxed);

        (matches, has_profanity)
    }
}
