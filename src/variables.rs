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