#![allow(dead_code, unused_variables)]

pub fn code() {
    use std::sync::Arc;
    use std::{sync::Mutex, thread};

    #[derive(Debug)]
    struct Database {
        connections: Vec<u32>,
    }

    impl Database {
        fn new() -> Database {
            Database {
                connections: vec![],
            }
        }
        fn connect(&mut self, id: u32) {
            self.connections.push(id);
        }
    }

    let db = Arc::new(Mutex::new(Database::new()));

    let mut handles = vec![];

    for i in 0..10 {
        let db = Arc::clone(&db);
        let handle = thread::spawn(move || {
            let mut db_lock = db.lock().unwrap();
            db_lock.connect(i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let db_lock = db.lock().unwrap();

    println!("{db_lock:?}");
}

pub fn acquiring_locks() {
    // In order to access a value wrapped in a mutex, it's lock has to be acquired.
    // Fix the code by acquiring the lock at appropriate places.
    // use std::sync::{Arc, Mutex};
    // use std::thread;
    //
    // struct Wrapper {
    //     value: i32,
    // }
    //
    // impl Wrapper {
    //     fn new() -> Self {
    //         Wrapper { value: 0 }
    //     }
    //     fn add(&mut self, to_add: i32) {
    //         self.value += to_add;
    //     }
    // }
    //
    // // calculate sum of range 1..=40000 using four threads
    // let sum = Arc::new(Mutex::new(Wrapper::new()));
    // let mut handles = Vec::new();
    // for i in 0..4 {
    //     let sum_clone = Arc::clone(&sum);
    //     let handle = thread::spawn(move || {
    //         let mut sum = 0;
    //         let start = i * 10000 + 1;
    //         for num in start..start + 10000 {
    //             sum += num;
    //         }
    //         // Acquire lock and add sum to sum_clone
    //     });
    //     handles.push(handle);
    // }
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    // // Acquire lock and print the sum value
    // println!("Sum of range 1..=40000 : {}");

    use std::sync::{Arc, Mutex};
    use std::thread;

    struct Wrapper {
        value: i32,
    }

    impl Wrapper {
        fn new() -> Self {
            Wrapper { value: 0 }
        }
        fn add(&mut self, to_add: i32) {
            self.value += to_add;
        }
    }

    // calculate sum of range 1..=40000 using four threads
    let sum = Arc::new(Mutex::new(Wrapper::new()));
    let mut handles = Vec::new();
    for i in 0..4 {
        let sum_clone = Arc::clone(&sum);
        let handle = thread::spawn(move || {
            let mut sum = 0;
            let start = i * 10000 + 1;
            for num in start..start + 10000 {
                sum += num;
            }

            // Acquire lock and add sum to sum_clone
            let mut sum_lock = sum_clone.lock().unwrap();
            sum_lock.add(sum);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    // Acquire lock and print the sum value
    let sum_lock = sum.lock().unwrap();
    println!("Sum of range 1..=40000 : {}", sum_lock.value);
}

pub fn atomic_ref_count() {
    // Fix the code to make it compile.
    // use std::rc::Rc;
    // use std::sync::Mutex;
    // use std::thread;
    //
    // fn is_prime(num: u32) -> bool {
    //     for i in 2..=num / 2 {
    //         if num % i == 0 {
    //             return false;
    //         }
    //     }
    //     if num <= 1 {
    //         false
    //     } else {
    //         true
    //     }
    // }
    //
    // // list of all prime numbers less than 10000 using four threads
    // let mut primes = Rc::new(Mutex::new(Vec::new()));
    // let thread_count = 4;
    // let elements_per_thread = 10000 / thread_count;
    // let mut handles = Vec::new();
    // for i in 0..thread_count {
    //     let start = i * elements_per_thread;
    //     let list_clone = Rc::clone(&primes);
    //     let handle = thread::spawn(|| {
    //         for num in start..start + elements_per_thread {
    //             if is_prime(num) {
    //                 let mut lock = list_clone.lock().unwrap();
    //                 lock.push(num);
    //             }
    //         }
    //     });
    //     handles.push(handle);
    // }
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    // let lock = primes.lock().unwrap();
    // println!("Prime numbers:");
    // println!("{:?}", lock);
    // assert_eq!(lock.len(), 1229);

    use std::sync::Arc;
    use std::sync::Mutex;
    use std::thread;

    fn is_prime(num: u32) -> bool {
        for i in 2..=num / 2 {
            if num % i == 0 {
                return false;
            }
        }
        if num <= 1 { false } else { true }
    }

    // list of all prime numbers less than 10000 using four threads
    let primes = Arc::new(Mutex::new(Vec::new()));
    let thread_count = 4;
    let elements_per_thread = 10000 / thread_count;

    let mut handles = Vec::new();
    for i in 0..thread_count {
        let start = i * elements_per_thread;
        let list_clone = Arc::clone(&primes);
        let handle = thread::spawn(move || {
            for num in start..start + elements_per_thread {
                if is_prime(num) {
                    let mut lock = list_clone.lock().unwrap();
                    lock.push(num);
                }
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let lock = primes.lock().unwrap();
    println!("Prime numbers:");
    println!("{:?}", lock);
    assert_eq!(lock.len(), 1229);
}
