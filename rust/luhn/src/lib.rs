/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.trim().len() <= 1 {
        return false;
    }

    let mut is_code_contain_only_numbers = true;

    let numbers: Vec<u32> = code
        .chars()
        .filter(|char| *char != ' ')
        .map(|char| {
            let Some(number) = char::to_digit(char, 10) else {
                is_code_contain_only_numbers = false;
                return 99;
            };
            number
        })
        .collect();

    if !is_code_contain_only_numbers {
        return false;
    }

    let numbers_by_luna_algorithm: Vec<u32> = numbers
        .iter()
        .enumerate()
        .map(|(index, number)| {
            if numbers.len() % 2 == 0 {
                if index % 2 != 0 {
                    return *number;
                }
            } else if index % 2 == 0 {
                return *number;
            }

            if number * 2 > 9 {
                number * 2 - 9
            } else {
                number * 2
            }
        })
        .collect();

    numbers_by_luna_algorithm.iter().sum::<u32>() % 10 == 0
}
