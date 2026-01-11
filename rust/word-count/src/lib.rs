use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    words
        .replace(",", " ")
        .to_lowercase()
        .split_whitespace()
        .map(|word| {
            word.trim_matches('\'')
                .chars()
                .filter(|char| *char == '\'' || char.is_ascii_digit() || char.is_ascii_alphabetic())
                .collect()
        })
        .fold(HashMap::new(), |mut result, word| {
            *result.entry(word).or_insert(0) += 1;
            result
        })
}
