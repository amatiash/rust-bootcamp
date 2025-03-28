#![allow(dead_code, unused_variables)]

pub fn code() {
    // Example 1
    let player1 = String::from("player 1");
    let player2 = String::from("player 2");

    let result = first_turn(player1.as_str(), player2.as_str());

    // How does the borrow checker know result is not a dangling reference?
    println!("Player going first is: {}", result);

    // Example 2
    let player1 = String::from("player 1");
    {
        let player2 = String::from("player 2");
        let result = first_turn(player1.as_str(), player2.as_str());
        println!("Player going first is: {}", result);
    }

    // Example 3
    // let player1 = String::from("player 1");
    // let result;
    // {
    //     let player2 = String::from("player 2");
    //     result = first_turn(player1.as_str(), player2.as_str());
    // }
    // println!("Player going first is: {}", result);

    fn first_turn<'a>(p1: &'a str, p2: &'a str) -> &'a str {
        if rand::random() {
            p1
        } else {
            p2
        }
    }
}

pub fn helping_the_borrow_checker() {
    //noinspection DuplicatedCode
    // Make the code compile by completing the function signature.
    // fn longest(x: &str, y: &str) -> &str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }
    // 
    // let string1 = String::from("abcd");
    // let string2 = "xyz";
    // 
    // let result = longest(string1.as_str(), string2);
    // println!("The longest string is '{}'", result);
    
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}

pub fn complying_with_the_borrow_checker() {
    // Make the code compile. You can only shift one statement.
    // You can not shift variable declarations.
    // fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }
    // 
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is '{}'", result);

    //noinspection DuplicatedCode
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is '{}'", result);
    }
}
