pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return vec![];
    }

    if garden[0].is_empty() {
        return vec!["".to_string()];
    }

    let height = garden.len();
    let width = garden[0].len();
    let mut result: Vec<Vec<i8>> = vec![vec![]; height];

    for (index, row) in garden.iter().enumerate() {
        for char in row.chars() {
            match char {
                ' ' => result[index].push(0),
                '*' => result[index].push(-1),
                _ => (),
            }
        }
    }

    for row_index in 0..height {
        for column_index in 0..width {
            let value = result[row_index][column_index];
            if value != -1 {
                continue;
            }

            // left
            if column_index != 0 {
                let left_index = column_index - 1;

                if result[row_index][left_index] != -1 {
                    result[row_index][left_index] += 1;
                }
            }

            // right
            if column_index != width - 1 {
                let right_index = column_index + 1;

                if result[row_index][right_index] != -1 && right_index < width {
                    result[row_index][right_index] += 1;
                }
            }

            // bottom
            if row_index + 1 != height {
                let start_index = if column_index == 0 {
                    0
                } else {
                    column_index - 1
                };

                let end_index = if column_index + 1 == width {
                    column_index
                } else {
                    column_index + 1
                };

                for index in start_index..=end_index {
                    if result[row_index + 1][index] == -1 {
                        continue;
                    }
                    result[row_index + 1][index] += 1
                }
            }

            //top
            if row_index != 0 {
                let start_index = if column_index == 0 {
                    0
                } else {
                    column_index - 1
                };

                let end_index = if column_index + 1 == width {
                    column_index
                } else {
                    column_index + 1
                };

                for index in start_index..=end_index {
                    if result[row_index - 1][index] == -1 {
                        continue;
                    }
                    result[row_index - 1][index] += 1
                }
            }
        }
    }

    result
        .iter()
        .map(|row| {
            row.iter().fold(String::new(), |acc, number| match number {
                -1 => acc + "*",
                0 => acc + " ",
                _ => acc + &i8::to_string(number),
            })
        })
        .collect()
}
