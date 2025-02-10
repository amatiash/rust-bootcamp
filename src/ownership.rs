pub fn code() {
    // let s1 = String::from("Rust"); // heap allocated string
    // let s2 = s1.clone();
    // 
    // println!("s1 is: {s1}");
    // 
    // let x = 10;
    // let y = x;
    // println!("x is: {x}");

    // let s1 = String::from("Rust"); // heap allocated string
    // let s2 = s1.clone();
    // print_string(s1.clone());
    // let s3 = generate_string();
    // let s4 = add_to_string(s2);
    //
    // println!("s1 is: {s1}");
    // println!("s3 is: {s3}");
    // println!("s4 is: {s4}");
    //
    // let x = 10;
    // let y = x;
    // print_integer(x);
    // println!("x is: {x}");
    //
    // fn print_integer(i: i32) {
    //     println!("i is: {i}");
    // }
    //
    // fn add_to_string(mut p1: String) -> String {
    //     p1.push_str(" is awesome!");
    //     p1
    // }
    //
    // fn generate_string() -> String {
    //     String::from("Ferris")
    // }
    //
    // fn print_string(p1: String) {
    //     println!("{p1}");
    // } // p1 is dropped
}

pub fn moving_on_assignment() {
    // Something's missing. Fix the code so that it compiles.
    // let s1 = String::from("Rust");
    // let mut s2 = s1;
    // s2.push_str(" is an awesome language");
    // println!("String:\"{s1}\" is a substring of \"{s2}\"");

    let s1 = String::from("Rust");
    let mut s2 = s1.clone();
    s2.push_str(" is an awesome language");
    println!("String:\"{s1}\" is a substring of \"{s2}\"");
}

pub fn moving_on_assignment_2() {
    // Fix the code so that it compiles. Modify only one statement.
    // let mut my_str = String::from("Example");
    // let mut temp;
    // while my_str.len() > 0 {
    //     temp = my_str;
    //     println!("Length of temporary string is: {}", temp.len());
    //     my_str.pop();
    // }

    let mut my_str = String::from("Example");
    let mut temp;
    while my_str.len() > 0 {
        temp = my_str.clone();
        println!("Length of temporary string is: {}", temp.len());
        my_str.pop();
    }
}

pub fn moving_into_function () {
    // Fix the code so that it compiles.
    // let my_string = String::from("I love rust bootcamp 💕");
    // let occurrence_count = count_occurrences(my_string, 'o');
    // println!("The number of times 'o' appears in \"{my_string}\" = {occurrence_count}");
    // 
    // // this function counts the number of times a letter appears in a text
    // fn count_occurrences(text: String, letter: char) -> u32 {
    //     let mut res = 0;
    //     for ch in text.chars() {
    //         if ch == letter {
    //             res += 1;
    //         }
    //     }
    //     res
    // }

    let my_string = String::from("I love rust bootcamp 💕");
    let occurrence_count = count_occurrences(&my_string, 'o');
    println!("The number of times 'o' appears in \"{my_string}\" = {occurrence_count}");

    // this function counts the number of times a letter appears in a text
    fn count_occurrences(text: &String, letter: char) -> u32 {
        let mut res = 0;
        for ch in text.chars() {
            if ch == letter {
                res += 1;
            }
        }
        res
    }

}

pub fn moving_out_of_function() {
    // Make the following code compile by modifying only one statement.
    // let mut str1 = get_new_string();
    // println!("Printing through str1: {}", str1);
    // let mut str2 = str1;
    // println!("Printing through str2: {}", str2);
    // str1 = str2;
    // println!("Again printing through str1: {}", str1);
    // str2 = str1;
    // println!("Again printing through str2: {}", str2);
    // println!("Printing though both: {}, {}", str1, str2);
    // 
    // fn get_new_string() -> String {
    //     let new_string = String::from("I will master rust 🦀 🦀");
    //     new_string
    // } // string ownership is transferred to the calling function

    let mut str1 = get_new_string();
    println!("Printing through str1: {}", str1);
    let mut str2 = str1;
    println!("Printing through str2: {}", str2);
    str1 = str2;
    println!("Again printing through str1: {}", str1);
    str2 = str1.clone();
    println!("Again printing through str2: {}", str2);
    println!("Printing though both: {}, {}", str1, str2);

    fn get_new_string() -> String {
        let new_string = String::from("I will master rust 🦀 🦀");
        new_string
    } // string ownership is transferred to the calling function

}
