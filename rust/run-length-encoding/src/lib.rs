pub fn encode(source: &str) -> String {
    let mut result = String::new();
    let chars: Vec<&str> = source.split("").collect();

    let mut counter = 1;

    for index in 0..chars.len() - 1 {
        let current_char = chars[index];
        let next_char = chars[index + 1];

        if current_char == next_char {
            counter += 1;
            continue;
        }

        if counter == 1 {
            result.push_str(current_char);
        } else {
            result.push_str(&counter.to_string());
            result.push_str(current_char);
        }

        counter = 1;
    }

    result
}

pub fn decode(source: &str) -> String {
    let mut result = String::new();
    let chars: Vec<&str> = source.split("").collect();

    let mut number = String::new();

    for char in chars {
        match char.parse::<u8>() {
            Ok(_) => number.push_str(char),
            Err(_) => {
                result.push_str(&char.repeat(number.parse::<u32>().unwrap_or(1) as usize));
                number.clear();
            }
        }
    }

    println!("answe {}", result);
    result
}
