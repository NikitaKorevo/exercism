/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let isbn: String = isbn.replace('-', "");

    if isbn.len() != 10
        || !isbn
            .chars()
            .enumerate()
            .all(|(index, char)| char.is_numeric() || (index == 9 && char == 'X'))
    {
        return false;
    }

    let sum: u32 = isbn
        .chars()
        .rev()
        .enumerate()
        .fold(0, |acc, (index, char)| {
            acc + (index as u32 + 1)
                * match char {
                    'X' => 10,
                    char => char.to_digit(10).unwrap(),
                }
        });

    sum % 11 == 0
}
