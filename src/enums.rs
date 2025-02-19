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
