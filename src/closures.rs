#![allow(dead_code, unused_variables)]

pub fn code() {
    struct Credentials<T>
    where
        T: Fn(&str, &str) -> bool,
    {
        username: String,
        password: String,
        validator: T,
    }

    impl<T> Credentials<T>
    where
        T: Fn(&str, &str) -> bool,
    {
        fn is_valid(&self) -> bool {
            (self.validator)(&self.username, &self.password)
        }
    }

    fn validate_credentials(username: &str, password: &str) -> bool {
        !username.is_empty() && !password.is_empty()
    }

    fn get_default_creds<T>(f: T) -> Credentials<T>
    where
        T: Fn(&str, &str) -> bool,
    {
        Credentials {
            username: "guest".to_owned(),
            password: "password123!".to_owned(),
            validator: f,
        }
    }

    fn get_password_validator(
        min_len: usize,
        special_char: bool,
    ) -> Box<dyn Fn(&str, &str) -> bool> {
        if special_char {
            Box::new(move |_: &str, password: &str| {
                !password.len() >= min_len
                    && password.contains(['!', '@', '#', '$', '%', '^', '&', '*'])
            })
        } else {
            Box::new(move |_: &str, password: &str| !password.len() >= min_len)
        }
    }

    let validator = |username: &str, password: &str| !username.is_empty() && !password.is_empty();
    let weak_password = "password123!".to_owned();
    // Fn - Immutably borrow variables in environment.
    // FnMut - Mutably borrow variables in environment. Can change environment.
    // FnOnce - Take ownership of variables in environment. Can only be called once.
    let validator2 = |username: &str, password: &str| {
        !username.is_empty()
            && !password.is_empty()
            && password.len() > 8
            && password.contains(['!', '@', '#', '$', '%', '^', '&', '*'])
            && password != weak_password
    };
    println!("{weak_password}");
    let creds = Credentials {
        username: "admin".to_owned(),
        password: "password123!".to_owned(),
        validator: validator2,
    };
    println!("{}", creds.is_valid());
    let password_validator = get_password_validator(8, true);
    let default_creds = get_default_creds(password_validator);
}

pub fn defining_closures() {
    // Define the `double` closure & make the code execute successfully.
    // let double = ;
    // assert_eq!(double(5), 10);
    // assert_eq!(double(-10), -20);

    let double = |x| x * 2;
    assert_eq!(double(5), 10);
    assert_eq!(double(-10), -20);
}

pub fn struct_fields() {
    // Complete the struct definition & the implementation block.

    /*
    struct BinaryOperation<T, U>
    where
        T: Copy,
    {
        operand1: T,
        operand2: T,
        operation: U,
    }

    impl<T, U> BinaryOperation<T, U>
    where
        T: Copy,
    {
        fn calculate(&self) -> T {}
    }

    let multiply = BinaryOperation {
        operand1: 20,
        operand2: 6,
        operation: |a, b| a * b,
    };
    let divide = BinaryOperation {
        operand1: 22.0,
        operand2: 7.0,
        operation: |x, y| x / y,
    };
    println!(
        "{} x {} = {}",
        multiply.operand1,
        multiply.operand2,
        multiply.calculate()
    );
    println!(
        "{} / {} = {}",
        divide.operand1,
        divide.operand2,
        divide.calculate()
    );
     */

    struct BinaryOperation<T, U>
    where
        T: Copy,
        U: Fn(T, T) -> T,
    {
        operand1: T,
        operand2: T,
        operation: U,
    }

    impl<T, U> BinaryOperation<T, U>
    where
        T: Copy,
        U: Fn(T, T) -> T,
    {
        fn calculate(&self) -> T {
            (self.operation)(self.operand1, self.operand2)
        }
    }

    let multiply = BinaryOperation {
        operand1: 20,
        operand2: 6,
        operation: |a, b| a * b,
    };
    let divide = BinaryOperation {
        operand1: 22.0,
        operand2: 7.0,
        operation: |x, y| x / y,
    };
    println!(
        "{} x {} = {}",
        multiply.operand1,
        multiply.operand2,
        multiply.calculate()
    );
    println!(
        "{} / {} = {}",
        divide.operand1,
        divide.operand2,
        divide.calculate()
    );
}

pub fn mutably_capturing_environment() {
    // Something is wrong with the struct definition. Can you fix it?

    /*
    struct Manipulator<T>
    where
        T: Fn(),
    {
        operation: T,
    }

    impl<T> Manipulator<T>
    where
        T: Fn(),
    {
        fn manipulate(&mut self) {
            (self.operation)()
        }
    }

    let mut x = 0;
    let mut y = 100;
    let mut x_manipulator = Manipulator {
        operation: || x += 1,
    };
    let mut y_manipulator = Manipulator {
        operation: || y /= 5,
    };
    x_manipulator.manipulate();
    x_manipulator.manipulate();
    y_manipulator.manipulate();
    y_manipulator.manipulate();
    assert_eq!(x, 2);
    assert_eq!(y, 4);
    */

    struct Manipulator<T>
    where
        T: FnMut(),
    {
        operation: T,
    }

    impl<T> Manipulator<T>
    where
        T: FnMut(),
    {
        fn manipulate(&mut self) {
            (self.operation)()
        }
    }

    let mut x = 0;
    let mut y = 100;
    let mut x_manipulator = Manipulator {
        operation: || x += 1,
    };
    let mut y_manipulator = Manipulator {
        operation: || y /= 5,
    };
    x_manipulator.manipulate();
    x_manipulator.manipulate();
    y_manipulator.manipulate();
    y_manipulator.manipulate();
    assert_eq!(x, 2);
    assert_eq!(y, 4);
}

pub fn moving_into_closures() {
    // Make the code compile.

    /*
    let my_str = "Hi there!".to_owned();
    let substr = "here";
    let check_substr = move |sub: &str| my_str.contains(sub);
    let res = check_substr(substr);
    // Shift the statement below to somewhere else
    println!("String: {my_str}");
    println!("String contains {substr} : {res}");
    */

    let my_str = "Hi there!".to_owned();
    let substr = "here";
    println!("String: {my_str}");
    let check_substr = move |sub: &str| my_str.contains(sub);
    let res = check_substr(substr);
    println!("String contains {substr} : {res}");
}

pub fn passing_to_functions() {
    // Complete the function signature.
    // fn average<T, U>(nums: &[i32], add: T, div: U) -> f32
    // where
    // {
    //     let mut sum = 0;
    //     for num in nums {
    //         sum = add(sum, *num);
    //     }
    //     div(sum, nums.len() as i32)
    // }
    //
    // let add = |a, b| a + b;
    // let div = |a, b| a as f32 / b as f32;
    // let my_nums = [1, 2, 3, 4, 5];
    // let res = average(&my_nums, add, div);
    // println!("Average of {my_nums:?} = {res}");

    fn average<T, U>(nums: &[i32], add: T, div: U) -> f32
    where
        T: Fn(i32, i32) -> i32,
        U: Fn(i32, i32) -> f32,
    {
        let mut sum = 0;
        for num in nums {
            sum = add(sum, *num);
        }
        div(sum, nums.len() as i32)
    }

    let add = |a, b| a + b;
    let div = |a, b| a as f32 / b as f32;
    let my_nums = [1, 2, 3, 4, 5];
    let res = average(&my_nums, add, div);
    println!("Average of {my_nums:?} = {res}");
}

pub fn returning_from_functions_1() {
    // Fix the code to make it compile.
    // let adder1 = get_adder(-2);
    // let adder2 = get_adder(100);
    // assert_eq!(adder1(20), 18);
    // assert_eq!(adder2(10), 110);
    //
    // fn get_adder(to_add: i32) -> impl Fn(i32) -> i32 {
    //     |x| x + to_add
    // }

    let adder1 = get_adder(-2);
    let adder2 = get_adder(100);
    assert_eq!(adder1(20), 18);
    assert_eq!(adder2(10), 110);

    fn get_adder(to_add: i32) -> impl Fn(i32) -> i32 {
        move |x| x + to_add
    }
}

pub fn returning_from_functions_2() {
    // Fix the code to make it compile.
    // enum Operation {
    //     Add,
    //     Sub,
    //     Mul,
    // }
    //
    // fn get_implementation(operation: Operation, operand2: i32) -> impl Fn(i32) -> i32 {
    //     match operation {
    //         Operation::Add => move |x| x + operand2,
    //         Operation::Sub => move |x| x - operand2,
    //         Operation::Mul => move |x| x * operand2,
    //     }
    // }
    //
    // const OPERAND2: i32 = 5;
    // let adder = get_implementation(Operation::Add, OPERAND2);
    // let multiplier = get_implementation(Operation::Mul, OPERAND2);
    // assert_eq!(adder(10), 15);
    // assert_eq!(multiplier(10), 50);

    enum Operation {
        Add,
        Sub,
        Mul,
    }

    fn get_implementation(operation: Operation, operand2: i32) -> Box<dyn Fn(i32) -> i32> {
        match operation {
            Operation::Add => Box::new(move |x| x + operand2),
            Operation::Sub => Box::new(move |x| x - operand2),
            Operation::Mul => Box::new(move |x| x * operand2),
        }
    }

    const OPERAND2: i32 = 5;
    let adder = get_implementation(Operation::Add, OPERAND2);
    let multiplier = get_implementation(Operation::Mul, OPERAND2);
    assert_eq!(adder(10), 15);
    assert_eq!(multiplier(10), 50);
}
