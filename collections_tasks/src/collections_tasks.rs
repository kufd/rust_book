use std::collections::hash_map::Entry;
use std::collections::HashMap;

pub struct Company {
    departments: HashMap<String, Vec<String>>
}

impl Company {
    pub fn new() -> Company {
        Company {
            departments: HashMap::new()
        }
    }

    pub fn get_employees_from_department(&self, department: &str) -> Option<&Vec<String>> {
        self.departments.get(department)
    }

    pub fn get_all_employees(&self) -> Vec<String> {
        let mut all_employees: Vec<String> = Vec::new();

        for (_, department) in &self.departments {
            all_employees.extend(department.clone())
        }

        all_employees.sort();

        all_employees
    }

    pub fn add_employee(&mut self, department: &str, name: &str) {
        match self.departments.entry(String::from(department)) {
            Entry::Vacant(e) => { e.insert(vec![String::from(name)]); },
            Entry::Occupied(mut e) => { e.get_mut().push(String::from(name)); e.get_mut().sort(); }
        }

        // self.departments.entry(String::from(department)).or_insert(Vec::new()).push(String::from(name));
    }
}

//==========================

pub struct Word {
    word: String,
}
impl Word {
    pub fn of(word: &str) -> Word {
        Word {
            word: String::from(word)
        }
    }

    pub fn to_string(&self) -> &String {
        &self.word
    }

    pub fn to_pig_latin(&self) -> Word {
        if self.is_start_with_vowels() {
            return Word {
                word: self.word.clone()+"-hay"
            };
        }

        Word {
            word: String::from(&self.word[1..])+"-"+&self.word[0..1]+"ay"
        }
    }

    fn is_start_with_vowels(&self) -> bool {
        let vowels = ['a', 'e', 'i', 'o', 'u'];

        for vowel in &vowels {
            if self.word.starts_with(*vowel) {
                return true;
            }
        }

        false
    }
}

//==========================

pub fn get_median(list: &mut Vec<i32>) -> Option<i32> {
    let median_key = list.len()/2;

    list.sort();

    list.get(median_key).map(|number| *number)
}

pub fn get_mode(list: Vec<i32>) -> Option<i32> {
    let mut counter: HashMap<&i32, i32> = HashMap::new();

    for number in &list {
        let count = counter.entry(number).or_insert(0);
        *count += 1;
    }

    let mut result: Option<(&i32, i32)> = None;

    for entry in counter {
        result = result.or(Some(entry))
            .map(|current_entry| if current_entry.1 < entry.1 {entry} else {current_entry})
    }

    result.map(|entry| *entry.0)
}