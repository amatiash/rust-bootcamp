#![allow(dead_code, unused_variables)]

pub fn code() {
    struct Person {
        first_name: String,
        last_name: String,
        occupation: String,
    }

    // struct PersonIterator {
    //     values: Vec<String>,
    // }

    // impl Iterator for PersonIterator {
    //     type Item = String;

    //     fn next(&mut self) -> Option<Self::Item> {
    //         if self.values.is_empty() {
    //             return None
    //         }
    //         Some(self.values.remove(0))
    //     }
    // }

    // impl IntoIterator for Person {
    //     type Item = String;
    //     type IntoIter = PersonIterator;

    //     fn into_iter(self) -> Self::IntoIter {
    //         PersonIterator {
    //             values: vec![
    //                 self.first_name,
    //                 self.last_name,
    //                 self.occupation
    //             ]
    //         }
    //     }
    // }

    impl IntoIterator for Person {
        type Item = String;
        type IntoIter = std::vec::IntoIter<Self::Item>;

        fn into_iter(self) -> Self::IntoIter {
            vec![self.first_name, self.last_name, self.occupation].into_iter()
        }
    }

    let p = Person {
        first_name: "John".to_owned(),
        last_name: "Doe".to_owned(),
        occupation: "Software Engineer".to_owned(),
    };

    // let mut i = p.into_iter();

    // println!("{:?}", i.next());
    // println!("{:?}", i.next());
    // println!("{:?}", i.next());
    // println!("{:?}", i.next());

    for item in p {
        println!("{item}");
    }
}

pub fn into_iterator_1() {
    // Provide the trait implementations and make the code execute successfully.
    // struct Employee {
    //     first_name: String,
    //     last_name: String,
    //     id: String,
    // }
    //
    // struct EmployeeIter {
    //     state: Vec<String>,
    // }
    //
    // impl Iterator for EmployeeIter {
    //     type Item;
    //     fn next(&mut self) -> Option<Self::Item> {}
    // }
    //
    // impl IntoIterator for Employee {
    //     type Item;
    //     type IntoIter;
    //     fn into_iter(self) -> Self::IntoIter {}
    // }
    //
    // let employee = Employee {
    //     first_name: "Alice".to_owned(),
    //     last_name: "Smith".to_owned(),
    //     id: "ab123".to_owned(),
    // };
    // let mut emp_iter = employee.into_iter();
    // println!("First name: {}", emp_iter.next().unwrap());
    // println!("Last name: {}", emp_iter.next().unwrap());
    // println!("ID: {}", emp_iter.next().unwrap());
    // assert_eq!(emp_iter.next(), None);

    struct Employee {
        first_name: String,
        last_name: String,
        id: String,
    }

    struct EmployeeIter {
        state: Vec<String>,
    }

    impl Iterator for EmployeeIter {
        type Item = String;
        fn next(&mut self) -> Option<Self::Item> {
            if self.state.is_empty() {
                None
            } else {
                Some(self.state.remove(0))
            }

            // Or using pop:
            // self.state.pop()
        }
    }

    impl IntoIterator for Employee {
        type Item = String;
        type IntoIter = std::vec::IntoIter<Self::Item>;
        fn into_iter(self) -> Self::IntoIter {
            vec![self.first_name, self.last_name, self.id].into_iter()
        }
    }

    let employee = Employee {
        first_name: "Alice".to_owned(),
        last_name: "Smith".to_owned(),
        id: "ab123".to_owned(),
    };
    let mut emp_iter = employee.into_iter();
    println!("First name: {}", emp_iter.next().unwrap());
    println!("Last name: {}", emp_iter.next().unwrap());
    println!("ID: {}", emp_iter.next().unwrap());
    assert_eq!(emp_iter.next(), None);
}

pub fn into_iterator_2() {
    // Fix the code by completing the into_iter() method.
    // struct Employee {
    //     first_name: String,
    //     last_name: String,
    //     id: String,
    // }
    //
    // impl IntoIterator for Employee {
    //     type Item = String;
    //     type IntoIter = std::vec::IntoIter<String>;
    //     fn into_iter(self) -> Self::IntoIter {
    //         vec![
    //             format!("First name: {}", self.first_name),
    //             // do the same for last_name & id
    //         ]
    //     }
    // }
    //
    // let employee = Employee {
    //     first_name: "Alice".to_owned(),
    //     last_name: "Smith".to_owned(),
    //     id: "ab123".to_owned(),
    // };
    // println!("Employee Details:");
    // for detail in employee {
    //     println!("{detail}");
    // }

    struct Employee {
        first_name: String,
        last_name: String,
        id: String,
    }

    impl IntoIterator for Employee {
        type Item = String;
        type IntoIter = std::vec::IntoIter<String>;
        fn into_iter(self) -> Self::IntoIter {
            vec![
                format!("First name: {}", self.first_name),
                format!("Last name: {}", self.last_name),
                format!("ID: {}", self.id),
            ]
            .into_iter()
        }
    }

    let employee = Employee {
        first_name: "Alice".to_owned(),
        last_name: "Smith".to_owned(),
        id: "ab123".to_owned(),
    };
    println!("Employee Details:");
    for detail in employee {
        println!("{detail}");
    }
}

// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
// pub fn capitalize_first(input: &str) -> String {
//     let mut c = input.chars();
//     match c.next() {
//         None => String::new(),
//         Some(first) => ???,
//     }
// }

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
// pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
//     vec![]
// }

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
// pub fn capitalize_words_string(words: &[&str]) -> String {
//     String::new()
// }

// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => {
            let mut result = first.to_uppercase().collect::<String>();
            result.push_str(c.as_str());
            result
        }
    }

    // Alternative:
    // match c.next() {
    //     None => String::new(),
    //     Some(first) => format!("{}{}", first.to_uppercase(), c.as_str()),
    // }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|&word| capitalize_first(word)).collect()
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    words
        .iter()
        .map(|&word| capitalize_first(word))
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
