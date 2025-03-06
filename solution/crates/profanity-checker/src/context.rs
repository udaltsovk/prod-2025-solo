use lazy_static::lazy_static;
use phf::{phf_set, Set};

lazy_static! {
    static ref CONTEXT_TRIGGERS: Set<&'static str> = phf_set! {
        "полный", "сущий", "настоящий", "совсем", "очень", "крайне"
    };
}

pub struct ContextAnalysis {
    pub score: f32,
    pub triggers: Vec<String>,
}

pub fn analyze_context(words: &[&str]) -> f32 {
    let mut score = 0.0;

    for word in words {
        if CONTEXT_TRIGGERS.contains(&word.to_lowercase().as_str()) {
            score += 0.3;
        }
    }

    let unique_words = words.iter().collect::<std::collections::HashSet<_>>().len();
    let repetition_factor = 1.0 - (unique_words as f32 / words.len().max(1) as f32);
    score += repetition_factor * 0.5;

    score.min(1.0)
}
