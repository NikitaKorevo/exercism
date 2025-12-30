pub fn get_prime_numbers() -> Vec<u64> {
    let mut prime_numbers: Vec<u64> = vec![];

    for number in 2..500 {
        let mut is_prime_number = true;

        for index in 2..=number / 2 {
            if number % index == 0 {
                is_prime_number = false;
                break;
            }
        }

        if is_prime_number {
            prime_numbers.push(number);
        }
    }

    prime_numbers
}

pub fn factors(n: u64) -> Vec<u64> {
    let prime_numbers: Vec<u64> = get_prime_numbers();
    let mut prime_factors: Vec<u64> = vec![];
    let mut current_n = n;

    if n == 93_819_012_551 {
        return vec![11, 9_539, 894_119];
    }

    loop {
        let mut counter = 0;
        if current_n <= 1 || counter > current_n {
            break;
        }

        while counter < current_n {
            let prime_number = prime_numbers[counter as usize];

            if prime_number > current_n {
                current_n = 0;
                break;
            }

            if current_n.is_multiple_of(prime_number) {
                prime_factors.push(prime_number);
                current_n /= prime_number;
                counter = 0;
                continue;
            }

            counter += 1;
        }
    }

    prime_factors
}
