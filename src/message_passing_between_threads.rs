#![allow(dead_code, unused_variables)]

pub fn code() {
    use std::sync::mpsc;
    use std::thread;

    let (tx, rx) = mpsc::channel();

    let sentences = [
        "!dlroW olleH".to_owned(),
        ".tsurT eW tsuR nI".to_owned(),
        "!ytsuR teG s'teL".to_owned(),
        "!tsuB ro tsuR".to_owned(),
    ];

    for s in sentences {
        let tx_clone = tx.clone();

        thread::spawn(move || {
            let reversed: String = s.chars().rev().collect();
            tx_clone.send(reversed).unwrap();
        });
    }

    drop(tx);

    for sentence in rx {
        println!("{sentence}");
    }
}

pub fn message_passing_1() {
    // Fix the code to make it compile and produce the correct output.
    // use std::sync::mpsc::{self, Sender};
    // use std::thread;
    //
    // // calculate sum of numbers using specified number of threads
    // fn sum(nums: &[i32], thread_count: usize, tx) {
    //     let elements_per_thread = nums.len() / thread_count;
    //     let mut start_pos;
    //     let mut partition;
    //     for i in 0..thread_count - 1 {
    //         start_pos = i * elements_per_thread;
    //         partition = Vec::from(&nums[start_pos..start_pos + elements_per_thread]);
    //         let tx_clone = tx.clone();
    //         thread::spawn(move || {
    //             let mut sum = 0;
    //             for num in partition {
    //                 sum += num;
    //             }
    //         });
    //     }
    //     // sum the remaining elements using last thread
    //     partition = Vec::from(&nums[(thread_count - 1) * elements_per_thread..]);
    //     thread::spawn(move || {
    //         let mut sum = 0;
    //         for num in partition {
    //             sum += num;
    //         }
    //         tx.send(sum).unwrap()
    //     });
    // }
    //
    // let (tx, rx) = mpsc::channel();
    // let nums = [12, 43, 54, 43, 53, 52, 98, 89];
    // sum(&nums, 3, tx);
    // let mut res = 0;
    // for sum in rx {
    //     res += sum;
    // }
    // println!("Sum of numbers: {res}");

    use std::sync::mpsc::{self, Sender};
    use std::thread;

    let (tx, rx) = mpsc::channel();
    let nums = [12, 43, 54, 43, 53, 52, 98, 89];
    sum(&nums, 3, tx);
    let mut res = 0;
    for sum in rx {
        res += sum;
    }
    println!("Sum of numbers: {res}");

    // calculate sum of numbers using specified number of threads
    fn sum(nums: &[i32], thread_count: usize, tx: Sender<i32>) {
        let elements_per_thread = nums.len() / thread_count;
        let mut start_pos;
        let mut partition;
        for i in 0..thread_count - 1 {
            start_pos = i * elements_per_thread;
            partition = Vec::from(&nums[start_pos..start_pos + elements_per_thread]);
            let tx_clone = tx.clone();
            thread::spawn(move || {
                let mut sum = 0;
                for num in partition {
                    sum += num;
                }
            });
        }
        // sum the remaining elements using last thread
        partition = Vec::from(&nums[(thread_count - 1) * elements_per_thread..]);
        thread::spawn(move || {
            let mut sum = 0;
            for num in partition {
                sum += num;
            }
            tx.send(sum).unwrap()
        });
    }
}

pub fn message_passing_2() {
    // Fix the code to make it compile.
    // use std::sync::mpsc;
    // use std::thread;
    //
    // let sentences = [
    //     "!tpircs llehs a eb ot detnaw eh esuaceB ?tsuR nrael barC eht sirreF did yhW".to_owned(),
    //     "!sgel sih fo thgie lla htiw tsuR ni edoc nac eh - reksat-itlum etamitlu eht si barC eht sirreF".to_owned()
    // ];
    // let (tx, rx) = mpsc::channel();
    // for sentence in sentences {
    //     let tx_clone = tx.clone();
    //     thread::spawn(|| {
    //         let reversed = sentence.chars().rev().collect::<String>();
    //     });
    // }
    // drop(tx);
    // let printer = thread::spawn(|| {
    //     println!("Reversed sentences:");
    //     for sentence in rx {
    //         println!("{sentence}");
    //     }
    // });
    // printer.join().unwrap();

    use std::sync::mpsc;
    use std::thread;

    let sentences = [
        "!tpircs llehs a eb ot detnaw eh esuaceB ?tsuR nrael barC eht sirreF did yhW".to_owned(),
        "!sgel sih fo thgie lla htiw tsuR ni edoc nac eh - reksat-itlum etamitlu eht si barC eht sirreF".to_owned()
    ];
    let (tx, rx) = mpsc::channel();
    for sentence in sentences {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let reversed = sentence.chars().rev().collect::<String>();
            tx_clone.send(reversed).unwrap();
        });
    }
    drop(tx);
    let printer = thread::spawn(|| {
        println!("Reversed sentences:");
        for sentence in rx {
            println!("{sentence}");
        }
    });
    printer.join().unwrap();
}
