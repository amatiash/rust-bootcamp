#![allow(dead_code, unused_variables, unused_macros)]

pub fn code() {
    // See ../declarative_macros/* for the code examples.
}

pub fn introduction() {
    // Fix the code to make it compile.
    // macro_rules! my_macro {
    //     () => {
    //         println!("Check out my macro!");
    //     };
    // }
    //
    // my_macro();

    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }

    my_macro!();
}

pub fn scope_1() {
    // Everything seems correct, but the code does not compile. Maybe it has to do with the position of defining a macro.
    // my_macro!();
    // macro_rules! my_macro {
    //     () => {
    //         println!("Check out my macro!");
    //     };
    // }

    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
    my_macro!();
}

pub fn scope_2() {
    // Fix the code by bringing `my_macros` in scope (You have to mark `macros` module with something).
    // mod macros {
    //     macro_rules! my_macro {
    //         () => {
    //             println!("Check out my macro!");
    //         };
    //     }
    // }
    //
    // my_macro!();

    #[macro_use]
    mod macros {
        macro_rules! my_macro {
            () => {
                println!("Check out my macro!");
            };
        }
    }

    my_macro!();
}

pub fn multiple_matchers_1() {
    // Fix the code to make it compile. You can not remove anything.
    // #[rustfmt::skip]
    // macro_rules! my_macro {
    //     () => {
    //         println!("Check out my macro!");
    //     }
    //     ($val:expr) => {
    //         println!("Look at this other macro: {}", $val);
    //     }
    // }
    //
    // my_macro!();
    // my_macro!(7777);

    #[rustfmt::skip]
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
        ($val:expr) => {
            println!("Look at this other macro: {}", $val);
        };
    }

    my_macro!();
    my_macro!(7777);
}

pub fn repetition() {
    // Complete the definition of `sum`.
    // macro_rules! sum {
    //     () => {
    //         {
    //             let mut sum = 0;
    //             $( sum += $a; )+
    //         }
    //     };
    // }
    //
    // assert_eq!(sum!(1, 2, 3), 6);
    // assert_eq!(sum!(10u8, 20u8), 30);

    macro_rules! sum {
       ($( $a:expr ), +) => {
            {
                let mut sum = 0;
                $( sum += $a; )+
                sum
            }
        };
    }

    assert_eq!(sum!(1, 2, 3), 6);
    assert_eq!(sum!(10u8, 20u8), 30);
}

pub fn multiple_matchers_2() {
    // We are trying to create a macro called `vec2`, which has the same functionality as the `vec` macro.
    // Complete the transcriber for each matcher.
    // macro_rules! vec2 {
    //     () => {};
    //     ($($a:expr),+ $(,)?) => {{}};
    //     ($m:expr; $n:expr) => {{}};
    // }
}

macro_rules! vec2 {
    () => {
        Vec::new()
    };
    ($($a:expr),+ $(,)?) => {
        {
            let mut vec = Vec::new();
            $( vec.push($a); )+
            vec
        }
    };
    ($m:expr; $n:expr) => {
        {
            let mut vec = Vec::new();
            for _ in 0..$n {
                vec.push($m.clone());
            }
            vec
        }
    };
}

#[cfg(test)]
mod tests {

    #[test]
    fn creates_empty_vector() {
        let first: Vec<i32> = vec![];
        let second: Vec<i32> = vec2![];
        assert_eq!(first, second);
    }

    #[test]
    fn creates_vector_from_list() {
        assert_eq!(vec![1, 2, 3,], vec2![1, 2, 3,]);
        assert_eq!(vec!['a', 'b', 'b', 'a'], vec2!['a', 'b', 'b', 'a']);
    }

    #[test]
    fn creates_vector_with_repeating_element() {
        assert_eq!(vec![5; 10], vec2![5;10]);
    }
}