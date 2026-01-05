use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    26 == HashSet::<char>::from_iter(
        sentence
            .to_lowercase()
            .chars()
            .filter(|char| char.is_ascii_alphabetic()),
    )
    .len()
}
