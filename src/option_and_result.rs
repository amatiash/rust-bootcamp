pub fn code() {
    let username = get_username(1);
    if let Some(name) = username {
        println!("{name}");
    }

    fn get_username(user_id: u32) -> Option<String> {
        let query = format!("GET username FROM users WHERE id={user_id}");
        let db_result = query_db(query);
        db_result.ok()
    }

    fn query_db(query: String) -> Result<String, String> {
        if query.is_empty() {
            Err(String::from("Query string is empty!"))
        } else {
            Ok(String::from("Ferris"))
        }
    }
}

pub fn matching_option() {
    // Fix the code so that it compiles.
    // struct Point {
    //     x: i32,
    //     y: i32,
    // }
    //
    // let y: Option<Point> = Some(Point { x: 100, y: 200 });
    //
    // match y {
    //     Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
    //     _ => println!("no match"),
    // }
    // y; // Fix without deleting this line.

    struct Point {
        x: i32,
        y: i32,
    }

    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match &y {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("no match"),
    }
    #[allow(path_statements)]
    y; // Fix without deleting this line.
}

pub fn matching_option_2() {
    // Fix the code so that it compiles.
    // fn last_element(nums: &[i32]) -> Option<i32> {
    //     if nums.len() > 0 {
    //         Some(nums[nums.len() - 1])
    //     } else {
    //         None
    //     }
    // }
    //
    // let my_nums = [1, 1, 2, 3, 5, 8, 13];
    // match last_element(&my_nums) {
    //     Some => println!("Last element: {ele}"),
    //     None => println!("Empty array"),
    // }

    fn last_element(nums: &[i32]) -> Option<i32> {
        if nums.len() > 0 {
            Some(nums[nums.len() - 1])
        } else {
            None
        }
    }

    let my_nums = [1, 1, 2, 3, 5, 8, 13];
    match last_element(&my_nums) {
        Some(ele) => println!("Last element: {ele}"),
        None => println!("Empty array"),
    }
}

pub fn if_let() {
    // Fix the code so that it compiles.
    // struct User {
    //     id: u32,
    //     name: String,
    // }
    //
    // fn get_user_name(id: u32) -> Option<String> {
    //     let database = [
    //         User {id: 1, name: String::from("Alice")},
    //         User {id: 2, name: String::from("Bob")},
    //         User {id: 3, name: String::from("Cindy")}
    //     ];
    //     for user in database {
    //         if user.id == id {
    //             return Some(user.name)
    //         }
    //     }
    //     None
    // }
    //
    // let user_id = 3;
    // if Some(name) == get_user_name(user_id) {
    //     println!("User's name: {name}");
    // }

    struct User {
        id: u32,
        name: String,
    }

    fn get_user_name(id: u32) -> Option<String> {
        let database = [
            User {
                id: 1,
                name: String::from("Alice"),
            },
            User {
                id: 2,
                name: String::from("Bob"),
            },
            User {
                id: 3,
                name: String::from("Cindy"),
            },
        ];
        for user in database {
            if user.id == id {
                return Some(user.name);
            }
        }
        None
    }

    let user_id = 3;
    if let Some(name) = get_user_name(user_id) {
        println!("User's name: {name}");
    }
}

pub fn matching_result() {
    // Complete the match statement to check whether operation succeeded or not.
    // enum Operation {
    //     Add(i32, i32),
    //     Mul(i32, i32),
    //     Sub { first: i32, second: i32 },
    //     Div { dividend: i32, divisor: i32 },
    // }
    //
    // impl Operation {
    //     fn execute(self) -> Result<i32, String> {
    //         match self {
    //             Self::Add(a, b) => Ok(a + b),
    //             Self::Mul(a, b) => Ok(a * b),
    //             Self::Sub { first, second } => Ok(first - second),
    //             Self::Div { dividend, divisor } => {
    //                 if divisor == 0 {
    //                     Err(String::from("Can not divide by zero"))
    //                 } else {
    //                     Ok(dividend / divisor)
    //                 }
    //             }
    //         }
    //     }
    // }
    //
    // let user_input = Operation::Div {
    //     dividend: 20,
    //     divisor: 0,
    // };
    // match user_input.execute() {
    //     => println!("Result: {res}"),
    //     => println!("Error: {e}"),
    // }

    #[allow(dead_code)]
    enum Operation {
        Add(i32, i32),
        Mul(i32, i32),
        Sub { first: i32, second: i32 },
        Div { dividend: i32, divisor: i32 },
    }

    impl Operation {
        fn execute(self) -> Result<i32, String> {
            match self {
                Self::Add(a, b) => Ok(a + b),
                Self::Mul(a, b) => Ok(a * b),
                Self::Sub { first, second } => Ok(first - second),
                Self::Div { dividend, divisor } => {
                    if divisor == 0 {
                        Err(String::from("Can not divide by zero"))
                    } else {
                        Ok(dividend / divisor)
                    }
                }
            }
        }
    }

    let user_input = Operation::Div {
        dividend: 20,
        divisor: 0,
    };
    match user_input.execute() {
        Ok(res) => println!("Result: {res}"),
        Err(e) => println!("Error: {e}"),
    }
}

pub fn returning_result() {
    // Complete the function signature.
    // fn greet(name: &str) -> {
    //     if name.len() > 0 {
    //         println!("Hello {name}!");
    //         Ok(())
    //     } else {
    //         Err("Empty name provided".to_string())
    //     }
    // }
    //
    // let name = "Tom";
    // if let Err(e) = greet(name) {
    //     println!("Error: {e}");
    // }

    fn greet(name: &str) -> Result<(), String> {
        if name.len() > 0 {
            println!("Hello {name}!");
            Ok(())
        } else {
            Err("Empty name provided".to_string())
        }
    }

    let name = "Tom";
    if let Err(e) = greet(name) {
        println!("Error: {e}");
    }
}
