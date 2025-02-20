pub fn code() {
    // struct Product {
    //     name: String,
    //     category: ProductCategory,
    //     price: f32,
    //     in_stock: bool,
    // }
    //
    // enum ProductCategory {
    //     Books,
    //     Clothing,
    //     Electrics,
    // }
    //
    // enum Command {
    //     Undo,
    //     Redo,
    //     AddText(String),
    //     MoveCursor(i32, i32),
    //     Replace { from: String, to: String },
    // }
    //
    // impl Command {
    //     fn serialize(&self) -> String {
    //         match self {
    //             Command::Undo => String::from("{ \"cmd\": \"undo\" }"),
    //             Command::Redo => String::from("{ \"cmd\": \"redo\" }"),
    //             Command::AddText(s) => {
    //                 format!(
    //                     "{{ \
    //                     \"cmd\": \"add_text\", \
    //                     \"text\": \"{s}\" \
    //                 }}"
    //                 )
    //             }
    //             Command::MoveCursor(line, column) => {
    //                 format!(
    //                     "{{ \
    //                     \"cmd\": \"move_cursor\", \
    //                     \"line\": {line}, \
    //                     \"column\": {column} \
    //                 }}"
    //                 )
    //             }
    //             Command::Replace { from, to } => {
    //                 format!(
    //                     "{{ \
    //                     \"cmd\": \"replace\", \
    //                     \"from\": \"{from}\", \
    //                     \"to\": \"{to}\", \
    //                 }}"
    //                 )
    //             }
    //         }
    //     }
    // }
    //
    // let category = ProductCategory::Electrics;
    // let product = Product {
    //     name: String::from("TV"),
    //     category,
    //     price: 200.98,
    //     in_stock: true,
    // };
    //
    // let age = 35;
    //
    // match age {
    //     1 => println!("Happy 1st Birthday!"),
    //     13..=19 => println!("You are a teenager!"),
    //     x => println!("You are {x} years old!"),
    // }
    //
    // let cmd1 = Command::Undo;
    // let cmd2 = Command::AddText(String::from("test"));
    // let cmd3 = Command::MoveCursor(22, 0);
    // let cmd4 = Command::Replace {
    //     from: String::from("a"),
    //     to: String::from("b"),
    // };
    //
    // println!("{}", cmd1.serialize());
    // println!("{}", cmd2.serialize());
    // println!("{}", cmd3.serialize());
    // println!("{}", cmd4.serialize());
}

pub fn definition_1() {
    // Complete the code
    // #[derive(Debug)] // this line makes the enum variants printable!
    // enum Message {
    //     // Define a few types of messages as used below
    // }
    //
    // println!("{:?}", Message::Quit);
    // println!("{:?}", Message::Echo);
    // println!("{:?}", Message::Move);
    // println!("{:?}", Message::ChangeColor);

    #[derive(Debug)] // this line makes the enum variants printable!
    enum Message {
        Quit,
        Echo,
        Move,
        ChangeColor,
    }

    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}

pub fn definition_2() {
    // Complete the code
    // #[derive(Debug)] // this line makes the enum variants printable!
    // enum Message {
    //     // Define the different variants used below
    // }
    //
    // impl Message {
    //     fn call(&self) {
    //         println!("{:?}", self);
    //     }
    // }
    //
    // let messages = [
    //     Message::Move { x: 10, y: 30 },
    //     Message::Echo(String::from("hello world")),
    //     Message::ChangeColor(200, 255, 255),
    //     Message::Quit,
    // ];
    //
    // for message in &messages {
    //     message.call();
    // }

    #[allow(dead_code)]
    #[derive(Debug)] // this line makes the enum variants printable!
    enum Message {
        // Define the different variants used below
        Move { x: i32, y: i32 },
        Echo(String),
        ChangeColor(i32, i32, i32),
        Quit,
    }

    impl Message {
        fn call(&self) {
            println!("{:?}", self);
        }
    }

    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}

pub fn exhaustive_requirement() {
    // Make the following code compile.
    // If you score 50 or less, you fail.
    // marks scored out of 100
    // let marks = 75u8;
    // match marks {
    //     91..=100 => println!("You performed excellent!"),
    //     71..=90 => println!("You performed good :)"),
    //     51..=70 => println!("Your performance was average..."),
    //     0..=30 => println!("You failed. Better luck next time."),
    //     101..=u8::MAX => println!("Invalid marks!!!"),
    // }

    let marks = 75u8;
    match marks {
        91..=100 => println!("You performed excellent!"),
        71..=90 => println!("You performed good :)"),
        51..=70 => println!("Your performance was average..."),
        0..=50 => println!("You failed. Better luck next time."),
        101..=u8::MAX => println!("Invalid marks!!!"),
    }
}
pub fn match_expression() {
    // Fix the code so that it compiles.
    // let side_count = 5;
    // let message = match side_count {
    //     0 | 1 | 2 => "invalid shape",
    //     3 => "it's a triangle",
    //     4 => "it's a quadrilateral",
    //     5 => "it's a pentagon",
    //     6 => "it's a hexagon",
    //     _ => "i don't know the name, lol",
    // }
    // println!("{message}");

    let side_count = 5;
    let message = match side_count {
        0 | 1 | 2 => "invalid shape",
        3 => "it's a triangle",
        4 => "it's a quadrilateral",
        5 => "it's a pentagon",
        6 => "it's a hexagon",
        _ => "i don't know the name, lol",
    };
    println!("{message}");
}
pub fn matching_on_enums_1() {
    // Fix the code so that it compiles.
    // USD coin types
    // cent values: penny:1, nickel:5, dime: 10, quarter:25
    // enum Coin {
    //     Penny,
    //     Nickel,
    //     Dime,
    //     Quarter,
    // }
    // fn value_in_cents(coin: Coin) -> u8 {
    //     match coin {
    //         Coin::Penny => 1,
    //         Coin::Nickel => 5,
    //         Coin::Quarter => 25,
    //     }
    // }
    //
    // let piggy_bank = [Coin::Nickel, Coin::Penny, Coin::Dime, Coin::Penny];
    // let mut my_savings = 0;
    // for coin in piggy_bank {
    //     my_savings += value_in_cents(coin);
    // }
    // println!("My savings: {my_savings} cents");

    // Fix the code so that it compiles.
    // USD coin types
    // cent values: penny:1, nickel:5, dime: 10, quarter:25
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Quarter => 25,
            Coin::Dime => 10,
        }
    }

    let piggy_bank = [Coin::Nickel, Coin::Penny, Coin::Dime, Coin::Quarter];
    let mut my_savings = 0;
    for coin in piggy_bank {
        my_savings += value_in_cents(coin);
    }
    println!("My savings: {my_savings} cents");
}

pub fn matching_on_enums_2() {
    // Fix the code so that it compiles.
    // enum Operation {
    //     Add(u8, u8),
    //     Mul(u8, u8),
    //     Subtract { first: u8, second: u8 },
    //     Divide { dividend: u8, divisor: u8 },
    // }
    // impl Operation {
    //     fn result(&self) -> u8 {
    //         match self {
    //             Self::Add(a, b) => a + b, // notice Self can be used instead of Operation
    //             Self::Subtract { first, second } => first - second,
    //             Self::Divide { dividend, divisor } => dividend / divisor,
    //         }
    //     }
    // }
    //
    // let user_operation = Operation::Subtract {
    //     first: 75,
    //     second: 20,
    // };
    // println!("Result: {}", user_operation::result());

    #[allow(dead_code)]
    enum Operation {
        Add(u8, u8),
        Mul(u8, u8),
        Subtract { first: u8, second: u8 },
        Divide { dividend: u8, divisor: u8 },
    }

    impl Operation {
        fn result(&self) -> u8 {
            match self {
                Self::Add(a, b) => a + b, // notice Self can be used instead of Operation
                Self::Mul(a, b) => a * b,
                Self::Subtract { first, second } => first - second,
                Self::Divide { dividend, divisor } => dividend / divisor,
            }
        }
    }

    let user_operation = Operation::Subtract {
        first: 75,
        second: 20,
    };
    println!("Result: {}", user_operation.result());
}
