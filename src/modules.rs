#![allow(dead_code)]

pub fn visibility() {
    // Something's missing. Fix the code so that it compiles.
    // mod sausage_factory {
    //     // Don't let anybody outside of this module see this!
    //     fn get_secret_recipe() -> String {
    //         String::from("Ginger")
    //     }
    //
    //     fn make_sausage() {
    //         get_secret_recipe();
    //         println!("sausage!");
    //     }
    // }
    //
    // sausage_factory::make_sausage();

    mod sausage_factory {
        // Don't let anybody outside of this module see this!
        fn get_secret_recipe() -> String {
            String::from("Ginger")
        }

        pub fn make_sausage() {
            get_secret_recipe();
            println!("sausage!");
        }
    }

    sausage_factory::make_sausage();
}

pub fn bringing_item_into_scope() {
    // Complete this use statement
    // use ???
    //
    // match SystemTime::now().duration_since(UNIX_EPOCH) {
    //     Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
    //     Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    // }

    use std::time::{SystemTime, UNIX_EPOCH};

    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}

pub fn multi_file_projects_1() {
    // Complete the code by bringing the required items into scope.
    // mod days {
    //     pub enum WeekDay {
    //         Sunday,
    //         Monday,
    //         Tuesday,
    //         Wednesday,
    //         Thursday,
    //         Friday,
    //         Saturday,
    //     }
    //
    //     pub fn is_holiday(day: &WeekDay) -> bool {
    //         match day {
    //             WeekDay::Sunday | WeekDay::Saturday => true,
    //             _ => false,
    //         }
    //     }
    // }
    //
    // let today = WeekDay::Friday;
    // if is_holiday(&today) {
    //     println!("I can go out!");
    // } else {
    //     println!("I have to work today!");
    // }

    use days::*;

    // Complete the code by bringing the required items into scope.
    mod days {
        pub enum WeekDay {
            Sunday,
            Monday,
            Tuesday,
            Wednesday,
            Thursday,
            Friday,
            Saturday,
        }

        pub fn is_holiday(day: &WeekDay) -> bool {
            match day {
                WeekDay::Sunday | WeekDay::Saturday => true,
                _ => false,
            }
        }
    }

    let today = WeekDay::Friday;
    if is_holiday(&today) {
        println!("I can go out!");
    } else {
        println!("I have to work today!");
    }
}

pub fn multi_file_projects_2() {
    // Complete the code by use declarations.
    // mod student {
    //     pub mod operations {
    //         use super::Student; // using super to refer to parent module
    //
    //         pub fn assign_grade(student: &mut Student) {
    //             if student.marks >= 80 {
    //                 student.grade = 'A';
    //             } else if student.marks >= 60 {
    //                 student.grade = 'B';
    //             } else {
    //                 student.grade = 'C';
    //             }
    //         }
    //
    //     }
    //
    //     pub struct Student {
    //         pub name: String, // struct fields can also be made public
    //         pub marks: u8,
    //         pub grade: char,
    //     }
    //
    //     impl Student {
    //         // make methods/associated functions public in order to access from outside the module
    //         pub fn new(name: &str, marks: u8) -> Self {
    //             Self {
    //                 name: name.to_string(),
    //                 marks,
    //                 grade: 'X',
    //             }
    //         }
    //     }
    //
    // }
    //
    // let mut student = Student::new("Alice", 75);
    // assign_grade(&mut student);
    // println!("{} got {} grade", student.name, student.grade);

    // Complete the code by use declarations.
    use student::Student;
    use student::operations::*;

    mod student {
        pub mod operations {
            use super::Student; // using super to refer to parent module

            pub fn assign_grade(student: &mut Student) {
                if student.marks >= 80 {
                    student.grade = 'A';
                } else if student.marks >= 60 {
                    student.grade = 'B';
                } else {
                    student.grade = 'C';
                }
            }
        }

        pub struct Student {
            pub name: String, // struct fields can also be made public
            pub marks: u8,
            pub grade: char,
        }

        impl Student {
            // make methods/associated functions public in order to access from outside the module
            pub fn new(name: &str, marks: u8) -> Self {
                Self {
                    name: name.to_string(),
                    marks,
                    grade: 'X',
                }
            }
        }
    }

    fn main() {
        let mut student = Student::new("Alice", 75);
        assign_grade(&mut student);
        println!("{} got {} grade", student.name, student.grade);
    }
}

pub fn re_exporting() {
    // Make the code compile
    // mod delicious_snacks {
    //     // Fix these use statements
    //     use self::fruits::PEAR as ???
    //     use self::veggies::CUCUMBER as ???
    //
    //     mod fruits { // 'static just implies that reference will be valid throughout program execution
    //         pub const PEAR: &'static str = "Pear";
    //         pub const APPLE: &'static str = "Apple";
    //     }
    //
    //     mod veggies {
    //         pub const CUCUMBER: &'static str = "Cucumber";
    //         pub const CARROT: &'static str = "Carrot";
    //     }
    // }
    //
    // fn main() {
    //     println!(
    //         "favorite snacks: {} and {}",
    //         delicious_snacks::fruit,
    //         delicious_snacks::veggie
    //     );
    // }

    // Make the code compile
    mod delicious_snacks {
        // Fix these use statements
        pub use self::fruits::PEAR as fruit;
        pub use self::veggies::CUCUMBER as veggie;

        mod fruits {
            // 'static just implies that reference will be valid throughout program execution
            pub const PEAR: &'static str = "Pear";
            pub const APPLE: &'static str = "Apple";
        }

        mod veggies {
            pub const CUCUMBER: &'static str = "Cucumber";
            pub const CARROT: &'static str = "Carrot";
        }
    }

    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}
