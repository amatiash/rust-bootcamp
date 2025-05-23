#![allow(dead_code, unused_variables)]

pub fn code() {
    // use std::fs::File;
    //
    // let file = File::open("example.txt");
    //
    // let file = File::open("example.txt").unwrap();
    // let file = File::open("example.txt").expect("Failed to open file!");
    //
    // // Matching result
    // let file = match file {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("Failed to open file: {:?}", error);
    //     }
    // };
    //
    // // Returning a Result type
    // fn get_user_id(username: &str) -> Result<i32, String> {
    //     if username.is_empty() {
    //         Err("Username can not be empty".to_owned())
    //     } else {
    //         Ok(1)
    //     }
    // }
}

pub fn returning_result_1() {
    // // Rewrite the find_pos function.
    // // The return type must be Result<usize, Error>.
    // #[derive(Debug, PartialEq)]
    // pub enum Error {
    //     EmptyTextOrPattern, // either text or pattern (or both) is empty string
    //     TextLenSmall,       // text.len() < pattern.len()
    //     PatternNotFound,    // pattern not a substring of text
    // }
    // // below function finds the starting index of `pattern` in `text`
    // // if `pattern` is not found, it returns text.len()
    // pub fn find_pos(text: &str, pattern: &str) -> usize {
    //     let pattern_len = pattern.len();
    //     for start in 0..text.len() - pattern_len + 1 {
    //         if &text[start..start + pattern_len] == pattern {
    //             return start;
    //         }
    //     }
    //     text.len()
    // }
}

// Rewrite the find_pos function.
// The return type must be Result<usize, Error>.
#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyTextOrPattern, // either text or pattern (or both) is empty string
    TextLenSmall,       // text.len() < pattern.len()
    PatternNotFound,    // pattern not a substring of text
}
// below function finds the starting index of `pattern` in `text`
// if `pattern` is not found, it returns text.len()
pub fn find_pos(text: &str, pattern: &str) -> Result<usize, Error> {
    if text.is_empty() || pattern.is_empty() {
        return Err(Error::EmptyTextOrPattern);
    }
    let pattern_len = pattern.len();

    if text.len() < pattern_len {
        return Err(Error::TextLenSmall);
    }

    for start in 0..text.len() - pattern_len + 1 {
        if &text[start..start + pattern_len] == pattern {
            return Ok(start);
        }
    }
    // text.len()
    Err(Error::PatternNotFound)
}

pub fn returning_result_2() {
    // This function refuses to generate text to be printed on a nametag if
    // you pass it an empty string. It'd be nicer if it explained what the problem
    // was, instead of just sometimes returning `None`. Thankfully, Rust has a similar
    // construct to `Option` that can be used to express error conditions. Let's use it!
    // pub fn generate_nametag_text(name: String) -> Option<String> {
    //     if name.is_empty() {
    //         // Empty names aren't allowed.
    //         None
    //     } else {
    //         Some(format!("Hi! My name is {}", name))
    //     }
    // }
}

// This function refuses to generate text to be printed on a nametag if
// you pass it an empty string. It'd be nicer if it explained what the problem
// was, instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Option` that can be used to express error conditions. Let's use it!
pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        // Empty names aren't allowed.
        Err(String::from("`name` was empty; it must be nonempty."))
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

pub fn returning_result_3() {
    // // Complete the `new` associated function to check for invalid input.
    // #[derive(PartialEq, Debug)]
    // struct PositiveNonzeroInteger(u64);
    //
    // #[derive(PartialEq, Debug)]
    // enum CreationError {
    //     Negative,
    //     Zero,
    // }
    //
    // impl PositiveNonzeroInteger {
    //     fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
    //         // Hmm...? Why is this only returning an Ok value?
    //         Ok(PositiveNonzeroInteger(value as u64))
    //     }
    // }
}

// Complete the `new` associated function to check for invalid input.
#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        if value == 0 {
            return Err(CreationError::Zero);
        }
        if value < 0 {
            return Err(CreationError::Negative);
        }

        Ok(PositiveNonzeroInteger(value as u64))
    }
}

pub fn unwrapping() {
    // Fix the code
    // #[derive(Debug)]
    // struct User {
    //     name: String,
    //     id: u32,
    // }
    //
    // fn fetch_user(id: u32) -> Result<User, String> {
    //     let database = vec![
    //         User { name: "Alice".to_string(), id: 1, },
    //         User { name: "Bob".to_string(), id: 2, },
    //         User { name: "Cindy".to_string(), id: 3, },
    //     ];
    //     for user in database {
    //         if user.id == id {
    //             return Ok(user);
    //         }
    //     }
    //     Err("User record not present".to_string())
    // }
    //
    // // `fetch_user` returns a Result type. Add the appropriate method call to extract the User instance and ignore the error case.
    // let user = fetch_user(2);
    // println!("User details: {user:?}");

    // Fix the code
    #[derive(Debug)]
    struct User {
        name: String,
        id: u32,
    }

    fn fetch_user(id: u32) -> Result<User, String> {
        let database = vec![
            User {
                name: "Alice".to_string(),
                id: 1,
            },
            User {
                name: "Bob".to_string(),
                id: 2,
            },
            User {
                name: "Cindy".to_string(),
                id: 3,
            },
        ];
        for user in database {
            if user.id == id {
                return Ok(user);
            }
        }
        Err("User record not present".to_string())
    }

    // `fetch_user` returns a Result type. Add the appropriate method call to extract the User instance and ignore the error case.
    let user = fetch_user(2).unwrap();
    println!("User details: {user:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_strings() {
        assert!(matches!(
            find_pos("", "pattern"),
            Err(Error::EmptyTextOrPattern)
        ));
        assert!(matches!(
            find_pos("text", ""),
            Err(Error::EmptyTextOrPattern)
        ));
        assert!(matches!(find_pos("", ""), Err(Error::EmptyTextOrPattern)));
    }

    #[test]
    fn small_text() {
        assert!(matches!(
            find_pos("hello", "hello there"),
            Err(Error::TextLenSmall)
        ));
    }

    #[test]
    fn pattern_not_present() {
        assert!(matches!(
            find_pos("hello", "bye"),
            Err(Error::PatternNotFound)
        ));
    }

    #[test]
    fn pattern_present() {
        assert!(matches!(find_pos("I luv Rust", "uv"), Ok(3)));
    }

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()),
            Ok("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
            // Don't change this line
            Err("`name` was empty; it must be nonempty.".into())
        );
    }

    #[test]
    fn test_creation() {
        assert!(PositiveNonzeroInteger::new(10).is_ok());
        assert_eq!(
            Err(CreationError::Negative),
            PositiveNonzeroInteger::new(-10)
        );
        assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
    }
}
