#![allow(dead_code, unused_variables)]

pub fn code() {
    // #![allow(dead_code)]
    // 
    // use std::{collections::HashMap, io, error::Error, fmt::Display};
    // 
    // #[derive(Debug)]
    // struct ParsePaymentInfoError {
    //     source: Option<Box<dyn Error>>,
    //     msg: Option<String>
    // }
    // 
    // impl Display for ParsePaymentInfoError {
    //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //         f.write_str("Parsing payment error: invalid payment info")
    //     }
    // }
    // 
    // impl Error for ParsePaymentInfoError {
    //     fn source(&self) -> Option<&(dyn Error + 'static)> {
    //         self.source.as_deref()
    //     }
    // }
    // 
    // fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParsePaymentInfoError> {
    //     let numbers = card
    //         .split(" ")
    //         .map(|s| {
    //             s.parse().map_err(|_| ParsePaymentInfoError {
    //                 source: None,
    //                 msg: Some(format!("{s:?} could not be parsed as u32"))
    //             })
    //         })
    //         .collect::<Result<Vec<u32>, _>>()
    //         .map_err(|e| ParsePaymentInfoError {
    //             source: Some(Box::new(e)),
    //             msg: Some(format!("Failed to parse input as numbers. Input: {card}"))
    //         })?;
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
    // fn parse_card(card: &str) -> Result<Card, ParsePaymentInfoError> {
    //     let mut numbers = parse_card_numbers(card)?;
    // 
    //     let len = numbers.len();
    //     let expected_len = 4;
    // 
    //     if len != expected_len {
    //         return Err(ParsePaymentInfoError {
    //             source: None,
    //             msg: Some(format!(
    //                 "Incorrect number of elements parsed. Expected {expected_len} but got {len}. Elements: {numbers:?}"
    //             ))
    //         })
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
    // #[derive(Debug)]
    // enum CreditCardError {
    //     InvalidInput(String),
    //     Other(Box<dyn Error>, String),
    // }
    // 
    // fn get_credit_card_info(
    //     credit_cards: &HashMap<&str, &str>,
    //     name: &str,
    // ) -> Result<Card, CreditCardError> {
    //     let card_string = credit_cards.get(name).ok_or(
    //         CreditCardError::InvalidInput(format!("No credit card was found for {name}."))
    //     )?;
    // 
    //     let card = parse_card(card_string)
    //         .map_err(|e| {
    //             CreditCardError::Other(Box::new(e), format!("{name}'s card could not be parsed."))
    //         })?;
    // 
    //     Ok(card)
    // }
    // 
    // fn main() {
    //     env_logger::init();
    // 
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
    //         Err(err) => {
    //             match &err {
    //                 CreditCardError::InvalidInput(msg) => println!("{msg}"),
    //                 CreditCardError::Other(_, _) => println!("\nSomething went wrong! Try again!")
    //             }
    // 
    //             log::error!("\n{err:?}");
    //         }
    //     }
    // }
}

pub fn converting_errors_1() {
    // Complete the code by implementing `From<ParseIntError>` for `AddError`.
    // use std::num::ParseIntError;
    // 
    // #[derive(Debug)]
    // enum AddError {
    //     ParseError(ParseIntError),
    //     Overflow,
    // }
    // 
    // impl From<ParseIntError> for AddError {}
    // 
    // fn add(num1: &str, num2: &str) -> Result<u8, AddError> {
    //     let num1 = num1.parse::<u8>()?;
    //     let num2 = num2.parse::<u8>()?;
    //     num1.checked_add(num2).ok_or(AddError::Overflow)
    // }
    // 
    // let (input1, input2) = ("23", "50");
    // match add(input1, input2) {
    //     Ok(res) => println!("{input1} + {input2} = {res}"),
    //     Err(e) => println!("Error: {e:?}"),
    // }

    // Complete the code by implementing `From<ParseIntError>` for `AddError`.
    use std::num::ParseIntError;
    
    #[derive(Debug)]
    enum AddError {
        ParseError(ParseIntError),
        Overflow,
    }
    
    impl From<ParseIntError> for AddError {
        fn from(err: ParseIntError) -> Self {
            AddError::ParseError(err)
        }
    }
    
    fn add(num1: &str, num2: &str) -> Result<u8, AddError> {
        let num1 = num1.parse::<u8>()?;
        let num2 = num2.parse::<u8>()?;
        num1.checked_add(num2).ok_or(AddError::Overflow)
    }
    
    let (input1, input2) = ("23", "50");
    match add(input1, input2) {
        Ok(res) => println!("{input1} + {input2} = {res}"),
        Err(e) => println!("Error: {e:?}"),
    }
}

pub fn converting_errors_2() {
    // Complete the `add` function by converting from ParseIntError to AddError using `map_err` combinator.
    // use std::num::ParseIntError;
    // 
    // #[derive(Debug)]
    // enum AddError {
    //     ParseError(ParseIntError),
    //     Overflow,
    // }
    // 
    // fn add(num1: &str, num2: &str) -> Result<u8, AddError> {
    //     let num1 = num1.parse::<u8>()?;
    //     let num2 = num2.parse::<u8>()?;
    //     num1.checked_add(num2).ok_or(AddError::Overflow)
    // }
    // 
    // let (input1, input2) = ("23", "50");
    // match add(input1, input2) {
    //     Ok(res) => println!("{input1} + {input2} = {res}"),
    //     Err(e) => println!("Error: {e:?}"),
    // }

    // Complete the `add` function by converting from ParseIntError to AddError using `map_err` combinator.
    use std::num::ParseIntError;
    
    #[derive(Debug)]
    enum AddError {
        ParseError(ParseIntError),
        Overflow,
    }
    
    fn add(num1: &str, num2: &str) -> Result<u8, AddError> {
        let num1 = num1.parse::<u8>().map_err(|e| AddError::ParseError(e))?;
        let num2 = num2.parse::<u8>().map_err(|e| AddError::ParseError(e))?;
        num1.checked_add(num2).ok_or(AddError::Overflow)
    }
    
    let (input1, input2) = ("23", "50");
    match add(input1, input2) {
        Ok(res) => println!("{input1} + {input2} = {res}"),
        Err(e) => println!("Error: {e:?}"),
    }
}

pub fn error_trait() {
    // Complete the code and make the tests pass by implementing std::error::Error for CalculationError
    // pub enum CalculationError {
    //     InvalidOperation(char),
    //     InvalidOperand(String),
    //     DivideByZero { dividend: i8 },
    //     Overflow,
    // }
    // 
    // pub fn calculate(num1: &str, num2: &str, operation: char) -> Result<i8, CalculationError> {
    //     let num1 = num1
    //         .parse::<i8>()
    //         .map_err(|_| CalculationError::InvalidOperand(num1.to_owned()))?;
    //     let num2 = num2
    //         .parse::<i8>()
    //         .map_err(|_| CalculationError::InvalidOperand(num2.to_owned()))?;
    //     match operation {
    //         '+' => num1.checked_add(num2).ok_or(CalculationError::Overflow),
    //         '-' => num1.checked_sub(num2).ok_or(CalculationError::Overflow),
    //         '*' => num1.checked_mul(num2).ok_or(CalculationError::Overflow),
    //         '/' => {
    //             if num2 == 0 {
    //                 return Err(CalculationError::DivideByZero { dividend: num1 });
    //             }
    //             num1.checked_div(num2).ok_or(CalculationError::Overflow)
    //         }
    //         _ => Err(CalculationError::InvalidOperation(operation)),
    //     }
    // }
    //
}

// Complete the code and make the tests pass by implementing std::error::Error for CalculationError
#[derive(Debug)]
pub enum CalculationError {
    InvalidOperation(char),
    InvalidOperand(String),
    DivideByZero { dividend: i8 },
    Overflow,
}

impl std::fmt::Display for CalculationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CalculationError::InvalidOperation(op) => {
                write!(f, "{op} is not a valid operation. Allowed: +,-,/,*")
            }
            CalculationError::InvalidOperand(operand) => {
                write!(f, "{operand} is not a valid integer in range [-128, 127]")
            }
            CalculationError::DivideByZero { dividend } => {
                write!(f, "Can not divide by zero. Attempting to divide {dividend} by 0")
            }
            CalculationError::Overflow => write!(f, "Overflow while performing the operation"),
        }
    }
}

impl std::error::Error for CalculationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

pub fn calculate(num1: &str, num2: &str, operation: char) -> Result<i8, CalculationError> {
    let num1 = num1
        .parse::<i8>()
        .map_err(|_| CalculationError::InvalidOperand(num1.to_owned()))?;
    let num2 = num2
        .parse::<i8>()
        .map_err(|_| CalculationError::InvalidOperand(num2.to_owned()))?;
    match operation {
        '+' => num1.checked_add(num2).ok_or(CalculationError::Overflow),
        '-' => num1.checked_sub(num2).ok_or(CalculationError::Overflow),
        '*' => num1.checked_mul(num2).ok_or(CalculationError::Overflow),
        '/' => {
            if num2 == 0 {
                return Err(CalculationError::DivideByZero { dividend: num1 });
            }
            num1.checked_div(num2).ok_or(CalculationError::Overflow)
        }
        _ => Err(CalculationError::InvalidOperation(operation)),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    struct Error {
        e: Box<dyn std::error::Error>,
    }
    impl Error {
        // presence of this method ensures that std::error::Error is satisfied for CalculationError
        fn new(err: CalculationError) -> Self {
            Self { e: Box::new(err) }
        }
    }

    #[test]
    fn invalid_operation() {
        let res1 = calculate("12", "20", '$');
        let res2 = calculate("45", "43", '^');
        match (res1, res2) {
            (Err(e1), Err(e2)) => {
                assert_eq!(
                    "$ is not a valid operation. Allowed: +,-,/,*",
                    format!("{}", e1)
                );
                assert_eq!(
                    "^ is not a valid operation. Allowed: +,-,/,*",
                    format!("{}", e2)
                );
            }
            _ => panic!("Error expected!"),
        }
    }

    #[test]
    fn invalid_operand() {
        let res1 = calculate("ab", "3r", '+');
        let res2 = calculate("45", "4.23", '^');
        match (res1, res2) {
            (Err(e1), Err(e2)) => {
                assert_eq!(
                    "ab is not a valid integer in range [-128, 127]",
                    format!("{}", e1)
                );
                assert_eq!(
                    "4.23 is not a valid integer in range [-128, 127]",
                    format!("{}", e2)
                );
            }
            _ => panic!("Error expected!"),
        }
    }

    #[test]
    fn divide_by_zero() {
        let res1 = calculate("45", "0", '/');
        let res2 = calculate("70", "0", '/');
        match (res1, res2) {
            (Err(e1), Err(e2)) => {
                assert_eq!(
                    "Can not divide by zero. Attempting to divide 45 by 0",
                    format!("{}", e1)
                );
                assert_eq!(
                    "Can not divide by zero. Attempting to divide 70 by 0",
                    format!("{}", e2)
                );
            }
            _ => panic!("Error expected!"),
        }
    }

    #[test]
    fn overflow() {
        let res = calculate("120", "120", '+');
        match res {
            Err(e) => {
                assert_eq!("Overflow while performing the operation", format!("{}", e));
            }
            _ => panic!("Error expected!"),
        }
    }
}
