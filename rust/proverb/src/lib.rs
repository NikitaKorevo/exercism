fn get_proverb_part(first_word: &str, second_word: &str) -> String {
    format!("For want of a {} the {} was lost.", first_word, second_word)
}

fn get_proverb_end(word: &str) -> String {
    format!("And all for the want of a {}.", word)
}

pub fn build_proverb(list: &[&str]) -> String {
    let mut result = vec![];

    if list.is_empty() {
        return "".to_string();
    }

    if list.len() == 1 {
        return get_proverb_end(list.last().unwrap());
    }

    for number in 0..list.len() - 1 {
        result.push(get_proverb_part(list[number], list[number + 1]));
    }

    result.push(get_proverb_end(list.first().unwrap()));
    result.join("\n")
}
