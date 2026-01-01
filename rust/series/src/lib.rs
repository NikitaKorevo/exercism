pub fn series(digits: &str, len: usize) -> Vec<String> {
    digits
        .chars()
        .map(|char| char.to_string())
        .collect::<Vec<String>>()
        .windows(len)
        .map(|group| group.join(""))
        .collect()
}
