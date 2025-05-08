#![allow(dead_code, unused_variables)]

pub fn code() {
    // #![allow(dead_code)]
    // 
    // use std::{collections::HashMap, io, error::Error, fmt::Display};
    // 
    // struct ParsePaymentInfoError {
    //     source: Option<Box<dyn Error>>,
    //     msg: String
    // }
    // 
    // impl std::fmt::Debug for ParsePaymentInfoError {
    //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //         write!(f, "{self} \n\t{}", self.msg)?;
    // 
    //         if let Some(e) = self.source.as_ref() {
    //             write!(f, "\n\nCaused by:\n\t{e:?}")?;
    //         }
    // 
    //         Ok(())
    //     }
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
    //                 msg: format!("{s:?} could not be parsed as u32")
    //             })
    //         })
    //         .collect::<Result<Vec<u32>, _>>()
    //         .map_err(|e| ParsePaymentInfoError {
    //             source: Some(Box::new(e)),
    //             msg: format!("Failed to parse input as numbers. Input: {card}")
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
    //             msg: format!(
    //                 "Incorrect number of elements parsed. Expected {expected_len} but got {len}. Elements: {numbers:?}"
    //             )
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
    // enum CreditCardError {
    //     InvalidInput(String),
    //     Other(Box<dyn Error>, String),
    // }
    // 
    // impl std::fmt::Debug for CreditCardError {
    //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //         match self {
    //             Self::InvalidInput(msg) => write!(f, "{self}\n{msg}"),
    //             Self::Other(e, msg) => write!(f, "{self}\n{msg}\n\nCaused by:\n\t{e:?}"),
    //         }
    //     }
    // }
    // 
    // impl Display for CreditCardError {
    //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //         f.write_str("Credit card error: Could not retrieve credit card.")
    //     }
    // }
    // 
    // impl Error for CreditCardError {
    //     fn source(&self) -> Option<&(dyn Error + 'static)> {
    //         match self {
    //             CreditCardError::InvalidInput(_) => None,
    //             CreditCardError::Other(e, _) => {
    //                 Some(e.as_ref())
    //             }
    //         }
    //     }
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

pub fn error_trait() {
    use std::{error::Error, fmt::{Display, Debug}};

    enum Month {
        Jan, Feb, March, April, May, June, July, Aug, Sept, Oct, Nov, Dec,
    }

    impl TryFrom<u8> for Month {
        type Error = MonthError;
        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                1 => Ok(Month::Jan),
                2 => Ok(Month::Feb),
                3 => Ok(Month::March),
                4 => Ok(Month::April),
                5 => Ok(Month::May),
                6 => Ok(Month::June),
                7 => Ok(Month::July),
                8 => Ok(Month::Aug),
                9 => Ok(Month::Sept),
                10 => Ok(Month::Oct),
                11 => Ok(Month::Nov),
                12 => Ok(Month::Dec),
                _ => Err(MonthError {
                    source: None,
                    msg: format!("{value} does not correspond to a month")
                })
            }
        }
    }

    //noinspection RsImplToString
    impl ToString for Month {
        fn to_string(&self) -> String {
            match self {
                Self::Jan => "Jan",
                Self::Feb => "Feb",
                Self::March => "March",
                Self::April => "April",
                Self::May => "May",
                Self::June => "June",
                Self::July => "July",
                Self::Aug => "Aug",
                Self::Sept => "Sept",
                Self::Oct => "Oct",
                Self::Nov => "Nov",
                Self::Dec => "Dec"
            }.to_string()
        }
    }

    struct MonthError {
        source: Option<Box<dyn Error>>,
        msg: String,
    }

    impl Error for MonthError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            self.source.as_deref()
        }
    }

    // Complete the implementation of `Display` & `Debug` for `MonthError`.
    // impl Display for MonthError {
    //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //         // write "Error converting input to months, check your input"
    //     }
    // }
    // impl Debug for MonthError {
    //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //         // if self.source is Some, write "{a}\nCaused by: {b:?}", a=self.msg, b=error stored inside Some
    //         // else, write self.msg to the Formatter
    //     }
    // }

    // Complete the implementation of `Display` & `Debug` for `MonthError`.
    impl Display for MonthError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // write "Error converting input to months, check your input"
            write!(f, "Error converting input to months, check your input: {}", self.msg)
        }
    }
    impl Debug for MonthError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            // if self.source is Some, write "{a}\nCaused by: {b:?}", a=self.msg, b=error stored inside Some
            // else, write self.msg to the Formatter
            if let Some(e) = self.source.as_ref() {
                write!(f, "{}\nCaused by: {:?}", self.msg, e)
            } else {
                write!(f, "{}", self.msg)
            }
        }
    }
    
    // Or
    // impl Display for MonthError {
    //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //         f.write_str("Error converting input to months, check your input")
    //     }
    // }
    // 
    // impl Debug for MonthError {
    //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    //         if let Some(err) = &self.source {
    //             f.write_fmt(format_args!("{}\nCaused by: {:?}", self.msg, err))
    //         } else {
    //             f.write_str(&self.msg)
    //         }
    //     }
    // }

    // function to convert a string to corresponding vector of owned month strings
    // e.g. "1 2 3 4" -> ["Jan", "Feb", "March", "April"]
    fn get_months(months: &str) -> Result<Vec<String>, MonthError> {
        let nums =
            months.split(' ')
                .into_iter()
                .map(|num_str| num_str.parse::<u8>()
                    .map_err(|e| MonthError { source: Some(Box::new(e)),
                        msg: format!("Can not parse {num_str} to u8")
                    }))
                .collect::<Result<Vec<u8>, _>>()
                .map_err(|e| MonthError {
                    source: Some(Box::new(e)),
                    msg: "Could not convert string to numbers".to_string()
                })?;
        let month_strs =
            nums.into_iter()
                .map(|num| Month::try_from(num))
                .collect::<Result<Vec<Month>, _>>()
                .map_err(|e| MonthError {
                    source: Some(Box::new(e)),
                    msg: "Could not convert nums to months".to_string()
                })?
                .into_iter()
                .map(|month| month.to_string())
                .collect::<Vec<String>>();
        Ok(month_strs)

    }

    fn convert_and_print(nums: &str) {
        match get_months(nums) {
            Ok(months) => println!("Months: {months:?}\n"),
            Err(e) => println!("{e:?}\n"),
        }
    }

    let input1 = "1 2 3 4 9 10";
    let input2 = "xyz 10 12";
    let input3 = "1 3 20";
    convert_and_print(input1);
    convert_and_print(input2);
    convert_and_print(input3);
}
