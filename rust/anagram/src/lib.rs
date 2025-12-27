use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&str]) -> HashSet<&'a str> {
    let mut result: HashSet<&'a str> = HashSet::new();
    let mut chars: Vec<_> = word.to_lowercase().chars().collect();

    chars.sort();

    let sorted_word = chars.iter().collect::<String>();

    for current_word in possible_anagrams {
        let mut chars: Vec<_> = current_word.to_lowercase().chars().collect();

        chars.sort();

        let sorted_current_word = chars.iter().collect::<String>();

        if current_word.to_lowercase() != word.to_lowercase() && sorted_word == sorted_current_word
        {
            result.insert(Box::leak(Box::new(current_word.to_string())));
        };
    }

    result
}
