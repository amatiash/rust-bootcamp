pub fn code() {
    // const MAX_PLAYERS: u8 = 10;
    // static CASINO_NAME: &str = "Rusty Casino";
    //
    // fn main() {
    //     let a = 10;
    //     let b = 10;
    //
    //     let c = CASINO_NAME;
    //     let d = CASINO_NAME;
    // }
}

pub fn constants() {
    // Fix the code so it compiles!
    // const NUMBER = 3;
    // println!("Number {}", NUMBER);

    const NUMBER: i8 = 3; // const NUMBER: i32 = 3;
    println!("Number {}", NUMBER);
}

pub fn statics() {
    // Fix the code so it compiles!
    // static LANGUAGE = "Golang";
    //
    // // These two initializations perform a string copy from same memory location
    // let lang1 = LANGUAGE;
    // let mut lang2 = LANGUAGE;
    //
    // lang2 = "Rust";
    // println!("I like {} more than {}!", lang2, lang1);

    static LANGUAGE: &str = "Golang";

    // These two initializations perform a string copy from same memory location
    let lang1 = LANGUAGE;
    let mut lang2 = LANGUAGE;

    println!("lang2: {}", lang2);

    lang2 = "Rust";
    println!("I like {} more than {}!", lang2, lang1);
}
