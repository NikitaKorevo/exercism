pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (0..limit).fold(0, |result, number| {
        if factors.iter().any(|factor| number.is_multiple_of(*factor)) {
            result + number
        } else {
            result
        }
    })
}
