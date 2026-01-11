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
                .fold(String::new(), |acc, char| match char {
                    '\'' => acc + &char.to_string(),
                    _ if char.is_ascii_digit() => acc + &char.to_string(),
                    _ if char.is_ascii_alphabetic() => acc + &char.to_string(),
                    _ => acc,
                })
        })
        .fold(HashMap::new(), |mut result, word| {
            result.insert(
                word.to_string(),
                result.get(&word).cloned().unwrap_or(0) + 1,
            );

            result
        })
}
