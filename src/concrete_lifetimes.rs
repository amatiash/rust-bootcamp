#![allow(dead_code, unused_variables)]

pub fn code() {
    // Example 1
    // let s1 = String::from("Let's Get Rusty!");
    // println!("s1: {s1}");
    // let s2 = s1;
    // println!("s1: {s1}");

    // Example 2
    // let r1;
    // {
    //     let s1 = String::from("Let's Get Rusty!");
    //     r1 = &s1;
    // }
    // println!("r1: {r1}");

    //Example 3 (non-lexical)
    // let mut s1 = String::from("Let's Get Rusty");
    // let r1 = &s1;
    // println!("r1: {r1}");
    // let r2 = &mut s1;
    // r2.push_str("!");
}

pub fn lifetimes_of_owned_values() {
    // Fix the code by shifting only one statement.
    // let str1 = "🦀".to_string();
    // let bytes = str1.into_bytes();
    // println!("A crab: {str1}");
    // println!("A crab represented in unicode: {bytes:?}");

    let str1 = "🦀".to_string();
    println!("A crab: {str1}");
    let bytes = str1.into_bytes();
    println!("A crab represented in unicode: {bytes:?}");
}

pub fn dangling_references() {
    // Fix the code.
    // let num_ref;
    // {
    //     // Shift below statement to appropriate location
    //     let num = 23;
    //     num_ref = &num;
    // }
    // println!("Reference points to {}", num_ref);

    let num_ref;
    let num = 23;
    {
        num_ref = &num;
    }
    println!("Reference points to {}", num_ref);
}

pub fn non_lexical_lifetimes() {
    // Fix the code by shifting only one statement.
    // let mut my_str = "Old String".to_owned();
    // let ref1 = &my_str;
    // let ref2 = &mut my_str;
    // ref2.replace_range(0..3, "New");
    // println!("{ref1}");
    // println!("{ref2}");

    let mut my_str = "Old String".to_owned();
    let ref1 = &my_str;
    println!("{ref1}");
    let ref2 = &mut my_str;
    ref2.replace_range(0..3, "New");
    println!("{ref2}");
}
