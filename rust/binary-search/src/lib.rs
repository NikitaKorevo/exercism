pub fn find(mut array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let mut index = 0;

    loop {
        if array.len() == 1 {
            if array[0] == key {
                return Some(index);
            } else {
                return None;
            }
        }

        let current_index = array.len() / 2;
        println!("{current_index}");

        if array[current_index] == key {
            return Some(index + current_index);
        }

        if array[current_index] > key {
            array = &array[0..current_index];
        } else {
            array = &array[current_index..];
            index += current_index;
        }
    }
}
