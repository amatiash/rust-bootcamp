#![allow(dead_code, unused_variables)]

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
