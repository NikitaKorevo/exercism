pub fn abbreviate(phrase: &str) -> String {
    phrase
        .replace("-", " ")
        .split_whitespace()
        .map(|word| {
            if word == word.to_uppercase() {
                word.get(0..1).unwrap().to_string()
            } else {
                word.chars()
                    .enumerate()
                    .filter(|(index, char)| {
                        (*index == 0 || !char.is_lowercase()) && !char.is_ascii_punctuation()
                    })
                    .map(|(_, char)| char.to_ascii_uppercase())
                    .collect::<String>()
            }
        })
        .collect::<String>()
}
