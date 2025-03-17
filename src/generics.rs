#![allow(dead_code, unused_variables)]
pub fn code() {
    struct BrowserCommand<T> {
        name: String,
        payload: T,
    }

    impl<T> BrowserCommand<T> {
        fn new(name: String, payload: T) -> Self {
            BrowserCommand { name, payload }
        }
        fn get_payload(&self) -> &T {
            &self.payload
        }
    }

    impl BrowserCommand<String> {
        fn print_payload(&self) {
            println!("{}", self.payload);
        }
    }

    let cmd1 = BrowserCommand::new(
        "navigate".to_owned(),
        "letsGetRusty".to_owned(),
    );
    let cmd2 = BrowserCommand::new("zoom".to_owned(), 200);
    cmd1.print_payload();
    let p1 = cmd1.get_payload();
    let p2 = cmd2.get_payload();

    serialize_payload(p1);
    serialize_payload(p2);

    fn serialize_payload<T>(payload: &T) -> String {
        // convert payload to JSON string...
        "placeholder".to_owned()
    }
}

pub fn generic_type_example() {
    // // Fix the code by annotating variable with the correct type.
    // let mut shopping_list: Vec<?> = Vec::new();
    // shopping_list.push("milk");

    // Fix the code by annotating variable with the correct type.
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}

pub fn defining_generic_types_1() {
    // // Define the generic struct Point.
    // let p1 = Point { x: 20, y: 10 };
    // let p2 = Point { x: 22.3, y: 3.14 };
    // println!("Point1: ({}, {})", p1.x, p1.y);
    // println!("Point2: ({}, {})", p2.x, p2.y);

    struct Point<T> {
        x: T,
        y: T,
    }
    let p1 = Point { x: 20, y: 10 };
    let p2 = Point { x: 22.3, y: 3.14 };
    println!("Point1: ({}, {})", p1.x, p1.y);
    println!("Point2: ({}, {})", p2.x, p2.y);
}

pub fn defining_generic_types_2() {
    // // Make the code compile by defining Operation enum with a single generic type.
    // let _op1 = Operation::Add(15u8, 10u8);
    // let _op2 = Operation::Mul(150, 23);
    // let _op3 = Operation::Sub {
    //     left: 120,
    //     right: 50,
    // };
    // let _op4 = Operation::Div {
    //     dividend: 10.23,
    //     divisor: 2.43,
    // };

    // Make the code compile by defining Operation enum with a single generic type.
    enum Operation<T> {
        Add(T, T),
        Mul(T, T),
        Sub { left: T, right: T },
        Div { dividend: T, divisor: T },
    }

    let _op1 = Operation::Add(15u8, 10u8);
    let _op2 = Operation::Mul(150, 23);
    let _op3 = Operation::Sub {
        left: 120,
        right: 50,
    };
    let _op4 = Operation::Div {
        dividend: 10.23,
        divisor: 2.43,
    };
}

pub fn implementation_blocks() {
    // // Implement the add method for Pair<i32> type.
    // struct Pair<T>(T, T);
    // let p1 = Pair(10, 23);
    // let addition = p1.add();
    // assert_eq!(addition, 33);

    // Implement the add method for Pair<i32> type.
    struct Pair<T>(T, T);
    
    impl Pair<i32> {
        fn add(&self) -> i32 {
            self.0 + self.1
        }
    }

    // Another solution
    // use std::ops::Add;
    // 
    // struct Pair<T>(T, T);
    // impl<T> Pair<T>
    // where T: Add<Output = T> + Copy
    // {
    //     fn add(&self) -> T {
    //         self.0 + self.1
    //     }
    // }

    let p1 = Pair(10, 23);
    let addition = p1.add();
    assert_eq!(addition, 33);
}

pub fn generic_functions() {
    // // Fix the code so that it compiles.
    // fn take_and_give_ownership(input: T) -> {
    //     input
    // }
    //
    // struct User {
    //     name: String,
    //     id: u32,
    // }
    // let str1 = String::from("Ferris the 🦀!");
    // let user1 = User {
    //     name: "Alice".to_string(),
    //     id: 199,
    // };
    // let _str2 = take_and_give_ownership(str1);
    // let _user2 = take_and_give_ownership(user1);

    // Fix the code so that it compiles.
    fn take_and_give_ownership<T>(input: T) -> T {
        input
    }

    struct User {
        name: String,
        id: u32,
    }
    let str1 = String::from("Ferris the 🦀!");
    let user1 = User {
        name: "Alice".to_string(),
        id: 199,
    };
    let _str2 = take_and_give_ownership(str1);
    let _user2 = take_and_give_ownership(user1);
}

// // Rewrite Wrapper struct so that it supports wrapping ANY type.
// struct Wrapper {
//     value: u32,
// }
//
// impl Wrapper {
//     pub fn new(value: u32) -> Self {
//         Wrapper { value }
//     }
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn store_u32_in_wrapper() {
//         assert_eq!(Wrapper::new(42).value, 42);
//     }
//
//     #[test]
//     fn store_str_in_wrapper() {
//         assert_eq!(Wrapper::new("Foo").value, "Foo");
//     }
// }

// Rewrite Wrapper struct so that it supports wrapping ANY type.
struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
