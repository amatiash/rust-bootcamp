pub fn code(){
    // // boolean
    // let b1: bool = true;
    //
    // // unsigned integers
    // let i1: u8 = 1;
    // let i2: u16 = 1;
    // let i3: u32 = 1;
    // let i4: u64 = 1;
    // let i6: u128 = 1;
    //
    // // signed integers
    // let i7: i8 = 1;
    // let i8: i16 = 1;
    // let i9: i32 = 1;
    // let i10: i64 = 1;
    // let i11: i128 = 1;
    //
    // // floating point numbers
    // let f1: f32 = 1.0;
    // let f2: f64 = 1.0;
    //
    // // platform specific integers
    // let p1: usize = 1;
    // let p2: isize = 1;
    //
    // // characters, &str, and String
    // let c1: char = 'c';
    // let s1: &str = "hello";
    // let s2: String = String::from("hello");
    //
    // // arrays
    // let a1 = [1, 2, 3, 4, 5];
    //
    // let i1 = a1[4];
    //
    // // tuples
    // let t1 = (1, 2, 3);
    // let t1 = (5, 5.0, "5");
    //
    // let s1 = t1.2;
    // let (i1, f1, s1) = t1;
    //
    // let unit = ();
    //
    // // Type aliasing
    // type Age = u8;
    //
    // let a1: Age = 57;
}

pub fn boolean() {
    // Something's missing. Fix the code so that it compiles.
    // let is_morning = true;
    // if is_morning {
    //     println!("Good morning!");
    // }
    //
    // let // Finish the rest of this line
    // if is_evening {
    //     println!("Good evening!");
    // }

    let is_morning = true;
    if is_morning {
        println!("Good morning!");
    }

    let is_evening = true; // Finish the rest of this line
    if is_evening {
        println!("Good evening!");
    }
}

pub fn unsigned_int() {
    // Do you really have that few friends?
    // Assign the correct value to the variable.
    // let number_of_friends: u32; // Don't change this line!
    // number_of_friends = -1;
    // println!("I have {} friends!", number_of_friends);

    let number_of_friends: u32; // Don't change this line!
    number_of_friends = 2; // any integer from 0 to 4294967295 is valid
    println!("I have {} friends!", number_of_friends);
}

pub fn signed_int() {
    // Make this program compile by replacing the variable type.
    // let number_of_stars: i32;
    // number_of_stars = 400_000_000_000; // The Milky Way has more stars than can fit in a 32-bit integer type!
    // println!("There are about {} stars in the Milky Way galaxy!", number_of_stars);

    let number_of_stars: i64;
    number_of_stars = 400_000_000_000; // The Milky Way has more stars than can fit in a 32-bit integer type!
    println!(
        "There are about {} stars in the Milky Way galaxy!",
        number_of_stars
    );
}

pub fn floating_point_numbers() {
    // Assign the correct data types to the variables.
    // let pi2: f32;
    // pi2 = 3.14;
    // println!("Pi is {}, correct to 2 decimal places.", pi2);
    //
    // // What if we want to be more precise with our floating-point numbers?
    //
    // let pi15: /* Give this variable a data type! */;
    // pi15 = 3.141592653589793;
    // println!("Pi is {}, correct to 15 decimal places.", pi15);

    let pi2: f32;
    pi2 = 3.14;
    println!("Pi is {}, correct to 2 decimal places.", pi2);

    // What if we want to be more precise with our floating-point numbers?

    let pi15: f64 /* Give this variable a data type! */;
    pi15 = std::f64::consts::PI; // pi15 = 3.141592653589793;
    println!("Pi is {}, correct to 15 decimal places.", pi15);
}

//noinspection DuplicatedCode
pub fn char() {
    // Fill in the rest of the line that has code missing!

    // let my_first_initial = 'C';
    //
    // if my_first_initial.is_alphabetic() {
    //     println!("Alphabetical!");
    // } else if my_first_initial.is_numeric() {
    //     println!("Numerical!");
    // } else {
    //     println!("Neither alphabetic nor numeric!");
    // }
    //
    // let // Finish this line like the example! What's your favorite character?
    // // Try a letter, try a number, try a special character, try a character
    // // from a different language than your own, try an emoji!
    //
    // if your_character.is_alphabetic() {
    //     println!("Alphabetical!");
    // } else if your_character.is_numeric() {
    //     println!("Numerical!");
    // } else {
    //     println!("Neither alphabetic nor numeric!");
    // }

    let my_first_initial = 'C';

    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    let your_character = 'ツ'; // Finish this line like the example! What's your favorite character?
                               // Try a letter, try a number, try a special character, try a character
                               // from a different language than your own, try an emoji!

    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}

pub fn string_types() {
    // Make this program compile without changing the variable type!

    // let answer: String = /* Your favorite color here */;
    // println!("My current favorite color is {}", answer);

    let answer: String = String::from("white"); // or "white".to_string();
    println!("My current favorite color is {}", answer);
}

pub fn arrays() {
    // Create an array with at least 10 elements in it.
    // let a = /* Your array here */;
    //
    // if a.len() >= 10 {
    //     println!("Wow, that's a big array!");
    // }

    let a = [0,1,2,3,4,5,6,7,8,9] /* Your array here */;

    if a.len() >= 10 {
        println!("Wow, that's a big array!");
    }
}

pub fn tuples() {
    // Destructure the `cat` tuple so that the println will work.
    // let cat = ("Furry McFurson", 3.5);
    // let /* Your pattern here */ = cat;
    //
    // println!("{} is {} years old.", name, age);

    let cat = ("Furry McFurson", 3.5);
    let (name, age) /* Your pattern here */ = cat;

    println!("{} is {} years old.", name, age);
}

pub fn type_aliasing(){
    // Add a type alias for our dogs so we can store their names and ages.
    // type /* Finish this line */
    //
    // let dog1: Dog = (String::from("Albert"), 3);
    // println!("My dog {} is {} years old.", dog1.0, dog1.1);
    //
    // let dog2: Dog = (String::from("Barker"), 5);
    // println!("My other dog {} is {} years old.", dog2.0, dog2.1);

    type Dog = (String, i8); /* Finish this line */

    let dog1: Dog = (String::from("Albert"), 3);
    println!("My dog {} is {} years old.", dog1.0, dog1.1);

    let dog2: Dog = (String::from("Barker"), 5);
    println!("My other dog {} is {} years old.", dog2.0, dog2.1);
}
