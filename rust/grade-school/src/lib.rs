use std::collections::{BTreeMap, BTreeSet};

pub struct School {
    names_by_grades: BTreeMap<u32, BTreeSet<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            names_by_grades: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let have_name = self
            .names_by_grades
            .clone()
            .into_values()
            .any(|name| name.contains(student));

        println!(
            "have name: {}, name: {}, grade: {}",
            have_name, student, grade
        );

        if let Some(names) = self.names_by_grades.get_mut(&grade) {
            if !have_name {
                println!("add {}", student);
                names.insert(student.to_string());
            }
        } else {
            println!("add {}", student);
            if !have_name {
                self.names_by_grades
                    .insert(grade, BTreeSet::from([student.to_string()]));
            }
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        self.names_by_grades.keys().cloned().collect::<Vec<u32>>()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.names_by_grades.get(&grade) {
            Some(date) => date.iter().cloned().collect(),
            None => vec![],
        }
    }
}
