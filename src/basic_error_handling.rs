#![allow(dead_code, unused_variables)]

pub fn code() {
    // #![allow(dead_code)]
    //
    // use std::{collections::HashMap, io, num::ParseIntError};
    //
    // fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParseIntError> {
    //     let numbers = card
    //         .split(" ")
    //         .map(|s| {
    //             s.parse()
    //         })
    //         .collect::<Result<Vec<u32>, _>>()?;
    //
    //     Ok(numbers)
    // }
    //
    // #[derive(Debug)]
    // struct Expiration {
    //     year: u32,
    //     month: u32
    // }
    //
    // #[derive(Debug)]
    // struct Card {
    //     number: u32,
    //     exp: Expiration,
    //     cvv: u32,
    // }
    //
    // fn parse_card(card: &str) -> Result<Card, String> {
    //     let mut numbers = parse_card_numbers(card).map_err(|e| e.to_string())?;
    //
    //     let len = numbers.len();
    //     let expected_len = 4;
    //
    //     if len != expected_len {
    //         return Err(format!(
    //             "Incorrect number of elements parsed. Expected {expected_len} but got {len}. Elements: {numbers:?}"
    //         ));
    //     }
    //
    //     let cvv = numbers.pop().unwrap();
    //     let year = numbers.pop().unwrap();
    //     let month = numbers.pop().unwrap();
    //     let number = numbers.pop().unwrap();
    //
    //     Ok(Card {
    //         number,
    //         exp: Expiration { year, month },
    //         cvv
    //     })
    // }
    //
    // fn get_credit_card_info(
    //     credit_cards: &HashMap<&str, &str>,
    //     name: &str,
    // ) -> Result<Card, String> {
    //     let card_string = credit_cards.get(name).ok_or(format!("No credit card was found for {name}."))?;
    //
    //     let card = parse_card(card_string)?;
    //
    //     Ok(card)
    // }
    //
    // fn main() {
    //     let credit_cards = HashMap::from([
    //         ("Amy", "1234567 12 16 123"),
    //         ("Tim", "1234567 0616 123"),
    //         ("Bob", "1234567 Dec 08 123"),
    //     ]);
    //
    //     println!("Enter Name: ");
    //
    //     let mut name = String::new();
    //
    //     io::stdin()
    //         .read_line(&mut name)
    //         .expect("Failed to read line");
    //
    //     let result = get_credit_card_info(&credit_cards, name.trim());
    //
    //     match result {
    //         Ok(card) => println!("\nCredit Card Info: {card:?}"),
    //         Err(err) => println!("{err}"),
    //     }
    // }
}

pub fn avoiding_unwrap() {
    // Modify `get_last` to handle all cases and make the program execute successfully.
    // fn get_last(nums: &mut Vec<i32>) -> i32 {
    //     nums.pop().unwrap()
    // }
    //
    // let mut vec1 = vec![1, 2, 3];
    // let mut vec2 = vec![];
    // assert!(matches!(get_last(&mut vec1), Ok(3)));
    // assert!(matches!(get_last(&mut vec2), Err("Empty vector")));

    // Modify `get_last` to handle all cases and make the program execute successfully.
    fn get_last(nums: &mut Vec<i32>) -> Result<i32, &str> {
        nums.pop().ok_or("Empty vector")
    }

    // or
    // fn get_last(nums: &mut Vec<i32>) -> Result<i32, &str> {
    //     if nums.len() == 0 {
    //         return Err("Empty vector");
    //     }
    //     Ok(nums.pop().unwrap())
    // }

    let mut vec1 = vec![1, 2, 3];
    let mut vec2 = vec![];
    assert!(matches!(get_last(&mut vec1), Ok(3)));
    assert!(matches!(get_last(&mut vec2), Err("Empty vector")));
}

pub fn error_propagating() {
    // Modify the functions to propagate the error instead of panicking.
    // fn factorial(n: u32) -> Result<u32, String> {
    //     if n == 0 {
    //         return Ok(1);
    //     } else if n > 12 {
    //         // Factorial of values > 12 would overflow u32, so return an error
    //         return Err(String::from("Input too large"));
    //     }
    //     let result = n * factorial(n - 1).unwrap();
    //     Ok(result)
    // }
    //
    // fn print_factorial(n: u32) -> Result<(), String> {
    //     let result = factorial(n).unwrap();
    //     println!("Factorial of {} is: {}", n, result);
    //     Ok(())
    // }
    //
    // let n = 13;
    // if let Err(err) = print_factorial(n) {
    //     eprintln!("Error calculating factorial of {}: {}", n, err);
    // }

    // Modify the functions to propagate the error instead of panicking.
    fn factorial(n: u32) -> Result<u32, String> {
        if n == 0 {
            return Ok(1);
        } else if n > 12 {
            // Factorial of values > 12 would overflow u32, so return an error
            return Err(String::from("Input too large"));
        }
        let result = n * factorial(n - 1)?;
        Ok(result)
    }

    fn print_factorial(n: u32) -> Result<(), String> {
        let result = factorial(n)?;
        println!("Factorial of {} is: {}", n, result);
        Ok(())
    }

    // let n = 13;
    // if let Err(err) = print_factorial(n) {
    //     eprintln!("Error calculating factorial of {}: {}", n, err);
    // }
}
