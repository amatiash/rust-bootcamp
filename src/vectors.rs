pub fn code (){
    // let mut v = Vec::new();
    // v.push(String::from("One"));
    // v.push(String::from("Two"));
    // v.push(String::from("Three"));
    //
    // let v2 = vec![1, 2, 3];
    //
    // let s = &v[0]; // can panic
    // // let s = v.remove(0);
    //
    // let s = v.get(0);
    //
    // if let Some(e) = s {
    //     println!("{e}");
    // }
    //
    // for s in &mut v {
    //     s.push_str("!");
    // }
    //
    // for s in &v {
    //     println!("{s}");
    // }
    //
    // let mut v3 = vec![];
    //
    // for s in v.into_iter() {
    //     v3.push(s);
    // }

    // let i = v.get(0);
}

pub fn pushing() {
    // Complete the function to make the program execute successfully.
    // fn append(nums, num) {
    // }
    // let mut nums = vec![1, 2, 5, 6];
    // append(&mut nums, 8);
    // append(&mut nums, 3);
    // assert_eq!(nums.len(), 6);

    fn append(nums: &mut Vec<i32>, num: i32) {
        nums.push(num);
    }
    let mut nums = vec![1, 2, 5, 6];
    append(&mut nums, 8);
    append(&mut nums, 3);
    assert_eq!(nums.len(), 6);
}

pub fn removing() {
    // Complete the function to make the program execute successfully.
    // fn remove_if_odd(nums, index) {
    //     if index > nums.len() {
    //         println!("Index out of bounds");
    //         return;
    //     }
    // }
    // let mut nums = vec![1, 2, 6, 9];
    // let nums_ref = &mut nums;
    // remove_if_odd(nums_ref, 0);
    // remove_if_odd(nums_ref, 1);
    // remove_if_odd(nums_ref, nums_ref.len() - 1);
    // assert_eq!(nums.len(), 2);

    // Complete the function to make the program execute successfully.
    fn remove_if_odd(nums: &mut Vec<i32>, index: usize) {
        if index > nums.len() {
            println!("Index out of bounds");
            return;
        }
        
        if (nums[index] % 2) != 0 {
            nums.remove(index);
        }
    }
    let mut nums = vec![1, 2, 6, 9];
    let nums_ref = &mut nums;
    remove_if_odd(nums_ref, 0);
    remove_if_odd(nums_ref, 1);
    remove_if_odd(nums_ref, nums_ref.len() - 1);
    assert_eq!(nums.len(), 2);
}

pub fn fetching() {
    // Fix the code so that it compiles.
    // let names = vec!["Alice", "Bob", "Cindy"];
    // let index = 2;
    // if names.get(index) {
    //     println!("{name} is present at index {index}");
    // } else {
    //     println!("invalid index {index}");
    // }

    let names = vec!["Alice", "Bob", "Cindy"];
    let index = 2;
    if let Some(name) = names.get(index) {
        println!("{name} is present at index {index}");
    } else {
        println!("invalid index {index}");
    }
}

pub fn iterating() {
    // Fix the code so that it compiles.
    // struct Student {
    //     name: String,
    //     marks: u8,
    // }
    // 
    // impl Student {
    //     fn new(name: &str, marks: u8) -> Self {
    //         Self {
    //             name: name.to_string(),
    //             marks,
    //         }
    //     }
    // }
    // 
    // let students = vec![
    //     Student::new("Harry", 75),
    //     Student::new("Hermione", 99),
    //     Student::new("Ron", 60),
    // ];
    // let mut grades = Vec::new();
    // for student in students {
    //     if student.marks > 80 {
    //         grades.push('A');
    //     } else if student.marks > 60 {
    //         grades.push('B');
    //     } else {
    //         grades.push('C');
    //     }
    // }
    // for i in 0..grades.len() {
    //     println!("{} got {}!", students[i].name, grades[i]);
    // }

    struct Student {
        name: String,
        marks: u8,
    }

    impl Student {
        fn new(name: &str, marks: u8) -> Self {
            Self {
                name: name.to_string(),
                marks,
            }
        }
    }

    let mut students = vec![
        Student::new("Harry", 75),
        Student::new("Hermione", 99),
        Student::new("Ron", 60),
    ];
    let mut grades = Vec::new();
    for student in &mut students {
        if student.marks > 80 {
            grades.push('A');
        } else if student.marks > 60 {
            grades.push('B');
        } else {
            grades.push('C');
        }
    }
    for i in 0..grades.len() {
        println!("{} got {}!", students[i].name, grades[i]);
    }
}

pub fn iterating_and_mutation() {
    // Fix the code so that it compiles.
    // struct Student {
    //     name: String,
    //     marks: u8,
    //     grade: char,
    // }
    // 
    // impl Student {
    //     fn new(name: &str, marks: u8) -> Self {
    //         Self {
    //             name: name.to_string(),
    //             marks,
    //             grade: 'X',
    //         }
    //     }
    // }
    // let students = vec![
    //     Student::new("Harry", 75),
    //     Student::new("Hermione", 99),
    //     Student::new("Ron", 60),
    // ];
    // for student in students {
    //     student.grade = if student.marks > 80 {
    //         'A'
    //     } else if student.marks > 60 {
    //         'B'
    //     } else {
    //         'C'
    //     };
    // }
    // for student in students {
    //     println!("{} got {}!", student.name, student.grade);
    // }

    struct Student {
        name: String,
        marks: u8,
        grade: char,
    }

    impl Student {
        fn new(name: &str, marks: u8) -> Self {
            Self {
                name: name.to_string(),
                marks,
                grade: 'X',
            }
        }
    }
    let mut students = vec![
        Student::new("Harry", 75),
        Student::new("Hermione", 99),
        Student::new("Ron", 60),
    ];
    for student in &mut students {
        student.grade = if student.marks > 80 {
            'A'
        } else if student.marks > 60 {
            'B'
        } else {
            'C'
        };
    }
    for student in students {
        println!("{} got {}!", student.name, student.grade);
    }
}