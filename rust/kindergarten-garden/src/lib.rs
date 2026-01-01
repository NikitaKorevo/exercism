fn get_plant(name: char) -> &'static str {
    match name {
        'G' => "grass",
        'C' => "clover",
        'R' => "radishes",
        'V' => "violets",
        _ => "",
    }
}

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let diagram: Vec<&str> = diagram.split('\n').collect();
    let first_row = diagram[0];
    let second_row = diagram[1];

    let get_data = |start_index: usize| {
        vec![
            get_plant(first_row.chars().nth(start_index).unwrap()),
            get_plant(first_row.chars().nth(start_index + 1).unwrap()),
            get_plant(second_row.chars().nth(start_index).unwrap()),
            get_plant(second_row.chars().nth(start_index + 1).unwrap()),
        ]
    };

    match student {
        "Alice" => get_data(0),
        "Bob" => get_data(2),
        "Charlie" => get_data(4),
        "David" => get_data(6),
        "Eve" => get_data(8),
        "Fred" => get_data(10),
        "Ginny" => get_data(12),
        "Harriet" => get_data(14),
        "Ileana" => get_data(16),
        "Joseph" => get_data(18),
        "Kincaid" => get_data(20),
        "Larry" => get_data(22),
        _ => vec![],
    }
}
