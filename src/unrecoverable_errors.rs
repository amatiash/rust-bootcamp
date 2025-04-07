#![allow(dead_code, unused_variables)]

pub fn code() {
    // panic!("Something went horribly wrong!");
    // let v = vec!["one", "two", "three"];
    // println!("{}", v[3]);
}

pub fn panicking() {
    // Make the program error out with appropriate message where required.
    // fn div(a: i32, b: i32) -> i32 {
    //     if b == 0 {
    //         // Panic with msg `divide by zero error`
    //     }
    //     a / b
    // }
    //
    // let _res = div(23, 0);

    fn div(a: i32, b: i32) -> i32 {
        if b == 0 {
            // Panic with msg `divide by zero error`
            panic!("divide by zero error");
        }
        a / b
    }

    // let _res = div(23, 0);
}
