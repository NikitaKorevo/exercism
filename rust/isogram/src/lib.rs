use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let candidate: String = candidate
        .to_lowercase()
        .chars()
        .filter(|char| char.is_ascii_alphabetic())
        .collect();
    let hash_set: HashSet<char> = HashSet::from_iter(candidate.chars());

    hash_set.len() == candidate.len()
}
