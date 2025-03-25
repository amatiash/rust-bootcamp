#![allow(dead_code, unused_variables)]

pub fn code() {
    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p1 = Point { x: 3, y: 1 };
    let p2 = Point { x: 3, y: 1 };
    let p3 = Point { x: 5, y: 5 };

    println!("{:?}", p1);
    println!("{}", p1 == p2);
    println!("{}", p1 == p3);
}

pub fn deriving_on_structs() {
    // Complete the code by deriving the required traits.
    // struct Point {
    //     x: i32,
    //     y: i32,
    // }
    //
    // let my_point = Point { x: 20, y: 10 };
    // let origin = Point::default();
    // println!("Origin: {origin:?}");
    // if my_point == origin {
    //     println!("Selected point is origin!");
    // } else {
    //     println!("Selected point: {my_point:?}");
    // }

    #[derive(Debug, Default, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    fn main() {
        let my_point = Point { x: 20, y: 10 };
        let origin = Point::default();
        println!("Origin: {origin:?}");
        if my_point == origin {
            println!("Selected point is origin!");
        } else {
            println!("Selected point: {my_point:?}");
        }
    }
}

pub fn deriving_on_enums() {
    // Only one trait needs to be derived. Can you figure out which?
    // enum Size {
    //     Small,
    //     Medium,
    //     Large,
    // }
    //
    // let my_size = Size::Small;
    // if my_size == Size::Small {
    //     println!("I can fit in any size!");
    // }

    #[derive(PartialEq)]
    enum Size {
        Small,
        Medium,
        Large,
    }

    let my_size = Size::Small;
    if my_size == Size::Small {
        println!("I can fit in any size!");
    }
}
