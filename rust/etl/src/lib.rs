use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result = BTreeMap::new();

    for (number, chars) in h {
        for char in chars {
            result.insert(char.to_ascii_lowercase(), *number);
        }
    }

    result
}
