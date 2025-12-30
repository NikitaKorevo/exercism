pub fn nth(n: u32) -> u32 {
    let mut simple_numbers: Vec<u32> = vec![];

    for number in 2..u32::MAX {
        let mut is_simple_numbers = true;

        for index in 2..=number / 2 {
            if number % index == 0 {
                is_simple_numbers = false;
                break;
            }
        }

        if is_simple_numbers {
            simple_numbers.push(number);

            if simple_numbers.len() > n as usize {
                break;
            }
        }
    }

    simple_numbers[n as usize]
}
