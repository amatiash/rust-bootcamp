#![allow(dead_code, unused_variables)]

pub fn code() {
    use std::thread;

    let s = "Let's Get Rusty!".to_owned();

    let handle = thread::spawn(move || {
        println!("{s}");
    });
}

pub fn moving() {
    // Fix the code to make it compile.
    // use std::thread;
    //
    // let msg = "Hello from spawned thread".to_owned();
    // let handle = thread::spawn(|| {
    //     println!("{msg}");
    // });
    // handle.join().unwrap();

    use std::thread;

    let msg = "Hello from spawned thread".to_owned();
    let handle = thread::spawn(move || {
        println!("{msg}");
    });
    handle.join().unwrap();
}
