pub fn brackets_are_balanced(string: &str) -> bool {
    let mut opening_brackets: Vec<char> = vec![];

    for char in string.chars() {
        println!("opening_brackets123 {:?}", opening_brackets);

        match char {
            '[' | '{' | '(' => opening_brackets.push(char),
            ']' => {
                if opening_brackets.pop() != Some('[') {
                    return false;
                };
            }
            '}' => {
                if opening_brackets.pop() != Some('{') {
                    return false;
                };
            }
            ')' => {
                if opening_brackets.pop() != Some('(') {
                    return false;
                };
            }
            _ => (),
        };
    }

    opening_brackets.is_empty()
}
