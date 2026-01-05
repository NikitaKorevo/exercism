use std::collections::HashSet;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let hash_set: HashSet<char> = HashSet::from_iter(
        sentence
            .to_lowercase()
            .chars()
            .filter(|char| char.is_ascii_alphabetic()),
    );
    hash_set.len() == 26
}
