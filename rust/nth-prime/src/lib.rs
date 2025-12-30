pub fn nth(n: u32) -> u32 {
    let mut prime_numbers: Vec<u32> = vec![];

    for number in 2..u32::MAX {
        let mut is_prime_number = true;

        for index in 2..=number / 2 {
            if number % index == 0 {
                is_prime_number = false;
                break;
            }
        }

        if is_prime_number {
            prime_numbers.push(number);

            if prime_numbers.len() > n as usize {
                break;
            }
        }
    }

    prime_numbers[n as usize]
}
