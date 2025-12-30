pub fn square(s: u32) -> u128 {
    if s == 0 || s >= 65 {
        panic!("")
    }

    (2..=s).fold(1, |acc, _| acc * 2)
}

pub fn total() -> u128 {
    (1..=64).fold(1, |acc, _| acc * 2) - 1
}
