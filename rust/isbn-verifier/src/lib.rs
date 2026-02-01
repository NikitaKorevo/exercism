/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn: String = isbn.replace('-', "");
    let points: Vec<u32> = isbn
        .chars()
        .enumerate()
        .filter_map(|(index, char)| match char {
            'X' if index == 9 => Some(10),
            '0'..='9' => char.to_digit(10).map(|number| number * (10 - index as u32)),
            _ => None,
        })
        .collect();

    if points.len() != 10 {
        return false;
    }

    points.iter().sum::<u32>() % 11 == 0
}
