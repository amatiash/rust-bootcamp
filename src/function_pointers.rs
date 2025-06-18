#![allow(dead_code, unused_variables)]

pub fn code() {
    let ten = 10;
    let greater_than = |x: &i32| *x > ten;
    let less_than = |x: &i32| *x < 20;

    let result = are_both_true(greater_than, less_than, &15);
    println!("{result}");

    fn greater_than_5(x: &i32) -> bool {
        *x > 5
    }

    // fn are_both_true<V>(f1: fn(&V) -> bool, f2: fn(&V) -> bool, item: &V) -> bool {
    //     f1(item) && f2(item)
    // }

    fn are_both_true<T, U, V>(f1: T, f2: U, item: &V) -> bool
    where
        T: Fn(&V) -> bool,
        U: Fn(&V) -> bool,
    {
        f1(item) && f2(item)
    }
}

pub fn as_parameters() {
    // Complete the function signature for `factorial`. It must not contain any generics/traits.
    // fn decrement(x: u32) -> u32 {
    //     x - 1
    // }
    //
    // fn multiply(x: u32, y: u32) -> u32 {
    //     x * y
    // }
    //
    // fn factorial(num, dec, mul) {
    //     let mut res = 1;
    //     let mut temp = num;
    //     while temp > 1 {
    //         res = mul(res, temp);
    //         temp = dec(temp);
    //     }
    //     res
    // }
    //
    // let num = 6;
    // let fact = factorial(num, decrement, multiply);
    // println!("{num}! = {fact}");

    fn decrement(x: u32) -> u32 {
        x - 1
    }

    fn multiply(x: u32, y: u32) -> u32 {
        x * y
    }

    // Complete the function signature for `factorial`. It must not contain any generics/traits.
    fn factorial(num: u32, dec: fn(u32) -> u32, mul: fn(u32, u32) -> u32) -> u32 {
        let mut res = 1;
        let mut temp = num;
        while temp > 1 {
            res = mul(res, temp);
            temp = dec(temp);
        }
        res
    }

    let num = 6;
    let fact = factorial(num, decrement, multiply);
    println!("{num}! = {fact}");
}

pub fn coercing_from_closures() {
    // Fix the code
    // fn invoker<T>(logic: fn(T), arg: T) {
    //     logic(arg);
    // }
    //
    // // Shift below declaration to somewhere else
    // let greeting = String::from("Nice to meet you");
    // let greet = |name| {
    //     println!("{greeting} {name}!");
    // };
    // invoker(greet, "Jenny");

    fn invoker<T>(logic: fn(T), arg: T) {
        logic(arg);
    }

    let greet = |name| {
        let greeting = String::from("Nice to meet you");
        println!("{greeting} {name}!");
    };
    invoker(greet, "Jenny");
}
