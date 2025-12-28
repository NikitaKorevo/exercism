#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }

    let first_list: String = first_list
        .iter()
        .map(|number: &i32| String::from("_") + &i32::to_string(number) + "_")
        .collect();
    let second_list: String = second_list
        .iter()
        .map(|number: &i32| String::from("_") + &i32::to_string(number) + "_")
        .collect();

    println!("{}", first_list);

    if first_list.contains(&second_list) {
        return Comparison::Superlist;
    }

    if second_list.contains(&first_list) {
        return Comparison::Sublist;
    }

    Comparison::Unequal
}
