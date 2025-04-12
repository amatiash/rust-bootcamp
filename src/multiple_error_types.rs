#![allow(dead_code, unused_variables)]

use std::fmt;
use std::{error, fs, num::ParseIntError};

pub fn code() {
    // let i = parse_file("example.txt");
    // match i {
    //     Ok(i) => println!("{i}"),
    //     Err(e) => {
    //         match e {
    //             ParseFileError::File => {
    //                 // ...
    //             },
    //             ParseFileError::Parse(e) => {
    //                 // ...
    //             }
    //         }
    //     }
    // }

    // fn parse_file(filename: &str) -> Result<i32, Box<dyn error::Error>> {
    //     let s = fs::read_to_string(filename)?;
    //     let i = s.parse()?;
    //     Ok(i)
    // }

    enum ParseFileError {
        File,
        Parse(ParseIntError),
    }

    fn parse_file(filename: &str) -> Result<i32, ParseFileError> {
        let s = fs::read_to_string(filename).map_err(|e| ParseFileError::File)?;
        let i = s.parse().map_err(|e| ParseFileError::Parse(e))?;
        Ok(i)
    }
}

// Update the return type of `main()` to make this compile
pub fn error_trait_object() -> Result<(), Box<dyn error::Error>> {
    // Don't change anything below this line.

    #[derive(PartialEq, Debug)]
    struct PositiveNonzeroInteger(u64);

    #[derive(PartialEq, Debug)]
    enum CreationError {
        Negative,
        Zero,
    }

    //noinspection DuplicatedCode
    impl PositiveNonzeroInteger {
        fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
            match value {
                x if x < 0 => Err(CreationError::Negative),
                x if x == 0 => Err(CreationError::Zero),
                x => Ok(PositiveNonzeroInteger(x as u64)),
            }
        }
    }

    // This is required so that `CreationError` can implement `error::Error`.
    impl fmt::Display for CreationError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let description = match *self {
                CreationError::Negative => "number is negative",
                CreationError::Zero => "number is zero",
            };
            f.write_str(description)
        }
    }

    impl error::Error for CreationError {}

    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

pub fn multiple_errors_1() {
    // Make the code compile by completing the add function.
    //
    // enum AddError {
    //     ParseError(ParseIntError),
    //     OverFlow,
    // }
    //
    // // parse input strings to u8's and return their sum
    // fn add(num1: &str, num2: &str) -> Result {
    //     let num1 = num1.parse::<u8>().map_err(|e| AddError::ParseError(e))?;
    //     let num2 = num2.parse::<u8>().map_err(|e| AddError::ParseError(e))?;
    //     // ok_or() transforms Option<T> -> Result<T,E> and takes value of type E as input
    //     let sum = num1.checked_add(num2).ok_or()?;
    //     Ok(sum)
    // }
    //
    // let (user_input1, user_input2) = ("23", "45");
    // match add(user_input1, user_input2) {
    //     Ok(sum) => println!("Sum = {sum}"),
    //     Err(e) => match e {
    //         AddError::OverFlow => println!("Sum > {}", u8::MAX),
    //         AddError::ParseError(e) => println!("Invalid input, parse error: {e:?}"),
    //     },
    // }

    enum AddError {
        ParseError(ParseIntError),
        OverFlow,
    }

    // Make the code compile by completing the add function.
    // parse input strings to u8's and return their sum
    fn add(num1: &str, num2: &str) -> Result<u8, AddError> {
        let num1 = num1.parse::<u8>().map_err(|e| AddError::ParseError(e))?;
        let num2 = num2.parse::<u8>().map_err(|e| AddError::ParseError(e))?;
        // ok_or() transforms Option<T> -> Result<T,E> and takes value of type E as input
        let sum = num1.checked_add(num2).ok_or(AddError::OverFlow)?;
        Ok(sum)
    }

    let (user_input1, user_input2) = ("23", "45");
    match add(user_input1, user_input2) {
        Ok(sum) => println!("Sum = {sum}"),
        Err(e) => match e {
            AddError::OverFlow => println!("Sum > {}", u8::MAX),
            AddError::ParseError(e) => println!("Invalid input, parse error: {e:?}"),
        },
    }
}

pub fn multiple_errors_2() {
    // Complete the code

    // // This is a custom error type that we will be using in `parse_pos_nonzero()`.
    // #[derive(PartialEq, Debug)]
    // enum ParsePosNonzeroError {
    //     Creation(CreationError),
    //     ParseInt(ParseIntError),
    // }
    //
    // impl ParsePosNonzeroError {
    //     fn from_creation(err: CreationError) -> ParsePosNonzeroError {
    //         ParsePosNonzeroError::Creation(err)
    //     }
    //     // Add another error conversion function here.
    //     // fn from_parseint...
    // }
    //
    // fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> {
    //     // Change this to return an appropriate error instead of panicking
    //     // when `parse()` returns an error.
    //     let x: i64 = s.parse().unwrap();
    //     PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from_creation)
    // }
    //
    // // Don't change anything below this line.
    //
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
    //         match value {
    //             x if x < 0 => Err(CreationError::Negative),
    //             x if x == 0 => Err(CreationError::Zero),
    //             x => Ok(PositiveNonzeroInteger(x as u64)),
    //         }
    //     }
    // }
}

// Complete the code

// This is a custom error type that we will be using in `parse_pos_nonzero()`.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::Creation(err)
    }
    // Add another error conversion function here.
    // fn from_parseint...
    fn from_parseint(err: ParseIntError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::ParseInt(err)
    }
}

fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> {
    // Change this to return an appropriate error instead of panicking
    // when `parse()` returns an error.
    let x: i64 = s.parse().map_err(ParsePosNonzeroError::from_parseint)?;
    PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from_creation)
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

//noinspection DuplicatedCode
impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        // We can't construct a ParseIntError, so we have to pattern match.
        assert!(matches!(
            parse_pos_nonzero("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_))
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            parse_pos_nonzero("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative))
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            parse_pos_nonzero("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero))
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42);
        assert!(x.is_ok());
        assert_eq!(parse_pos_nonzero("42"), Ok(x.unwrap()));
    }
}
