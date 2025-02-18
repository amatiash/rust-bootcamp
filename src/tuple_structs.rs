pub fn code(){
    // // tuples
    // let rbg_color = (255, 106, 0);
    // let cmyk_color = (0, 58, 100, 0);
    // 
    // // tuple structs
    // struct RGB(i32, i32, i32);
    // struct CMYK(i32, i32, i32, i32);
    // 
    // let color1 = RGB(255, 106, 0);
    // let color2 = CMYK(0, 58, 100, 0);
    // 
    // // unit-like structs
    // struct MyStruct;
}

pub fn definition(){
    // Complete the structure definition.

    // struct Point
    // 
    // impl Point {
    //     fn on_x_axis(&self) -> bool {
    //         self.1 == 0.0
    //     }
    //     fn on_y_axis(&self) -> bool {
    //         self.0 == 0.0
    //     }
    // }
    // 
    // let point = Point(0.0, 0.0);
    // if point.on_x_axis() && point.on_y_axis() {
    //     println!("Point is origin");
    // }

    struct Point(f32, f32);

    impl Point {
        fn on_x_axis(&self) -> bool {
            self.1 == 0.0
        }
        fn on_y_axis(&self) -> bool {
            self.0 == 0.0
        }
    }

    let point = Point(0.0, 0.0);
    if point.on_x_axis() && point.on_y_axis() {
        println!("Point is origin");
    }
}