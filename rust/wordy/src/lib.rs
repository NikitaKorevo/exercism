pub fn answer(command: &str) -> Option<i32> {
    let mut result = 0;
    let mut current_operation = "";
    let elements = command
        .trim_start_matches("What is ")
        .trim_end_matches('?')
        .replace("by", "");

    for (index, element) in elements.split_ascii_whitespace().enumerate() {
        if index % 2 == 0 {
            if let Ok(number) = element.parse::<i32>() {
                match current_operation {
                    "" => result += number,
                    "plus" => result += number,
                    "minus" => result -= number,
                    "multiplied" => result *= number,
                    "divided" => result /= number,
                    _ => return None,
                }
                current_operation = "";
            } else {
                return None;
            }
        } else {
            match element {
                "plus" | "minus" | "multiplied" | "divided" => current_operation = element,
                _ => return None,
            }
        }
    }

    if !current_operation.is_empty() {
        return None;
    }

    Some(result)
}
