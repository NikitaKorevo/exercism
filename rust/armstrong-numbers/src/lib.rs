pub fn is_armstrong_number(num: u32) -> bool {
    u32::to_string(&num)
        .chars()
        .map(|char| {
            char::to_digit(char, 10)
                .unwrap()
                .pow(u32::to_string(&num).len() as u32)
        })
        .sum::<u32>()
        == num
}
