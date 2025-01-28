pub fn code() {
    // // definition
    // let a = 5;
    //
    // println!("a is: {a}");
    //
    // // mutability
    // let mut b = 5;
    // b = 10;
    //
    // println!("b is: {b}");
    //
    // // shadowing
    // let c = 10;
    // let c = 20;
    //
    // println!("c is: {c}");
    //
    // // scope
    // let d = 30;
    //
    // {
    //     let d = 40;
    //     println!("inner d is: {d}");
    // }
    //
    // println!("d is: {d}")
}

pub fn fix_variable_definition() {
    // Fix the variable definition of 'x'
    // x = 5;
    // println!("x has the value {}", x);

    let x = 5;
    println!("x has the value {}", x);
}

pub fn fix_variable_definition_2() {
    // Fix the variable definition of 'x'
    // let x = 3;
    // println!("Number {}", x);
    // x = 5; // don't change this line
    // println!("Number {}", x);

    let mut x = 3;
    println!("Number {}", x);
    x = 5; // don't change this line
    println!("Number {}", x);
}

pub fn fix_code_with_shadowing() {
    // Fix this code with shadowing
    // let x = "three"; // don't change this line
    // println!("Spell a Number : {}", x);
    // x = 3; // don't rename this variable
    // println!("Number plus two is : {}", x + 2);

    let x = "three"; // don't change this line
    println!("Spell a Number : {}", x);
    let x = 3; // don't rename this variable
    println!("Number plus two is : {}", x + 2);
}