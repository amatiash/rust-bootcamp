#![allow(dead_code, unused_variables)]

pub fn code() {
    let mut s = "Let's Get Rusty".to_owned();

    let raw1 = &s as *const String;
    let raw2 = &mut s as *mut String;

    let address = 0x012345usize;
    let raw3 = address as *const String;

    unsafe {
        (*raw2).push_str("!!!");
        println!("raw1 is: {}", *raw1);
    }
}

pub fn immutable_raw_pointers() {
    // Fix the code to make it compile. You may not modify any statement.
    // let num = 123;
    // let ptr = &num as *const i32;
    // println!("{} stored at {:p}", *ptr, ptr);

    let num = 123;
    let ptr = &num as *const i32;

    unsafe {
        println!("{} stored at {:p}", *ptr, ptr);
    }
}

pub fn mutable_raw_pointers() {
    // Fix the code to make it compile.
    // print first 10 fibonacci numbers
    // let (mut a, mut b) = (1, 0);
    // let mut c = 0;
    // let ptr_a = &mut a as *const i32;
    // let ptr_b = &mut b as *const i32;
    // let ptr_c = &mut c as *const i32;
    // for _ in 0..10 {
    //     *ptr_c = *ptr_a + *ptr_b;
    //     println!("{}", *ptr_c);
    //     *ptr_a = *ptr_b;
    //     *ptr_b = *ptr_c;
    // }

    let (mut a, mut b) = (1, 0);
    let mut c = 0;
    let ptr_a = &mut a as *mut i32;
    let ptr_b = &mut b as *mut i32;
    let ptr_c = &mut c as *mut i32;

    unsafe {
        for _ in 0..10 {
            *ptr_c = *ptr_a + *ptr_b;
            println!("{}", *ptr_c);
            *ptr_a = *ptr_b;
            *ptr_b = *ptr_c;
        }
    }
}

pub fn multiple_pointers() {
    // Fix the code to make it compile.
    // macro_rules! ptr {
    //     ($type:ty, $var:ident) => {
    //         &mut $var as *mut $type
    //     };
    // }
    //
    // let mut x = 20;
    // let ptr1 = ptr!(i32, x);
    // let ptr2 = ptr!(i32, x);
    // println!("x: {x}");
    // *ptr1 = *ptr1 * 2;
    // *ptr2 = *ptr2 * 2;
    // *ptr2 = *ptr1 * 2;
    // println!("x * 8 = {x}");

    macro_rules! ptr {
        ($type:ty, $var:ident) => {
            &mut $var as *mut $type
        };
    }

    let mut x = 20;
    let ptr1 = ptr!(i32, x);
    let ptr2 = ptr!(i32, x);
    println!("x: {x}");
    unsafe {
        *ptr1 = *ptr1 * 2;
        *ptr2 = *ptr2 * 2;
        *ptr2 = *ptr1 * 2;
    }
    println!("x * 8 = {x}");
}
