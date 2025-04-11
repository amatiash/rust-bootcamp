#![allow(dead_code, unused_variables)]

pub fn code() {
    // use std::{fs, io};
    //
    // let first_line = read_first_line("example.txt");
    //
    // fn read_first_line(filename: &str) -> Result<Option<String>, io::Error> {
    //     fs::read_to_string(filename).map(|s| {
    //         s.lines().next().map(|s| s.to_owned())
    //     })
    // }
    //
    // fn read_first_line2(filename: &str) -> Option<String> {
    //     fs::read_to_string(filename).ok().and_then(|s| {
    //         s.lines().next().map(|s| s.to_owned())
    //     })
    // }
}

pub fn option_to_result() {
    // Fix the `fetch_last` function. Do not add any other statement.
    // fn fetch_last<T>(list: &mut Vec<T>) -> Option<T> {
    //     list.pop().ok_or()
    // }
    // 
    // let mut my_nums = Vec::<i32>::new();
    // match fetch_last(&mut my_nums) {
    //     Ok(ele) => println!("Last element: {ele}"),
    //     Err(error) => {
    //         println!("Error: {error}");
    //         assert_eq!(error, "Empty list".to_owned());
    //     }
    // }

    // Fix the `fetch_last` function. Do not add any other statement.
    fn fetch_last<T>(list: &mut Vec<T>) -> Result<T, String> {
        list.pop().ok_or("Empty list".to_owned())
    }
    
    let mut my_nums = Vec::<i32>::new();
    match fetch_last(&mut my_nums) {
        Ok(ele) => println!("Last element: {ele}"),
        Err(error) => {
            println!("Error: {error}");
            assert_eq!(error, "Empty list".to_owned());
        }
    }
}

pub fn result_to_option() {
    // Use `ok` combinator to convert Result to Option.
    // Do not add any statements anywhere.
    // fn add(num1: &str, num2: &str) -> Option<u8> {
    //     // Only modify the 2 statements below
    //     let num1 = num1.parse::<u8>();
    //     let num2 = num2.parse::<u8>();
    //     num1.checked_add(num2)
    // }
    // 
    // let (num1, num2) = ("4", "5");
    // if let Some(sum) = add("4", "5") {
    //     println!("{num1} + {num2} = {sum}");
    // }

    // Use `ok` combinator to convert Result to Option.
    // Do not add any statements anywhere.
    fn add(num1: &str, num2: &str) -> Option<u8> {
        // Only modify the 2 statements below
        let num1 = num1.parse::<u8>().ok()?;
        let num2 = num2.parse::<u8>().ok()?;
        num1.checked_add(num2)
    }
    
    let (num1, num2) = ("4", "5");
    if let Some(sum) = add("4", "5") {
        println!("{num1} + {num2} = {sum}");
    }
}
