#![allow(dead_code, unused_variables)]

pub fn code() {
    trait Vehicle: Paint {
        fn park(&self);
        fn get_default_color() -> String {
            "black".to_owned()
        }
    }

    trait Paint {
        fn paint(&self, color: String) {
            println!("painting object: {}", color);
        }
    }

    struct VehicleInfo {
        make: String,
        model: String,
        year: u16
    }

    struct Car {
        info: VehicleInfo
    }

    impl Vehicle for Car {
        fn park(&self) {
            println!("parking car!");
        }
    }

    impl Paint for Car {}

    struct Truck {
        info: VehicleInfo
    }

    impl Truck {
        fn unload(&self) {
            println!("unloading truck.")
        }
    }

    impl Vehicle for Truck {
        fn park(&self) {
            println!("parking truck!");
        }
    }

    impl Paint for Truck {}

    struct House {}

    impl Paint for House {
        fn paint(&self, color: String) {
            println!("painting house: {}", color);
        }
    }

    fn paint_red<T: Paint>(object: &T) {
        object.paint("red".to_owned());
    }

    fn paint_vehicle_red<T>(object: &T) where T: Vehicle {
        object.paint("red".to_owned());
    }

    fn create_paintable_object() -> impl Paint {
        House {}
    }

    let car = Car {
        info: VehicleInfo {
            make: "Honda".to_owned(),
            model: "Civic".to_owned(),
            year: 1995
        }
    };
    let house = House {};
    let object = create_paintable_object();

    paint_red(&car);
    paint_red(&house);
    paint_red(&object);

    paint_vehicle_red(&car);
}

pub fn implementing_supertraits() {
    // Make the code compile. Large should be the default size.
    /*
       Default trait is provided by standard library.
       Has one associated function: default() -> Self
    */
    // trait Bounded: Default {
    //     fn get_max() -> Self;
    //     fn get_min() -> Self;
    // }
    // 
    // enum Size {
    //     Small,
    //     Medium,
    //     Large,
    // }
    // 
    // impl Bounded for Size {
    //     fn get_max() -> Self {
    //         Self::Large
    //     }
    //     fn get_min() -> Self {
    //         Self::Small
    //     }
    // }
    // 
    // fn get_size_num(size: &Size) -> u8 {
    //     match size {
    //         Size::Small => 0,
    //         Size::Medium => 1,
    //         Size::Large => 2,
    //     }
    // }
    // 
    // let my_size = Size::Large;
    // let min_size_num = get_size_num(&Size::get_min());
    // let default_size_num = get_size_num(&Size::default());
    // let my_size_num = get_size_num(&my_size);
    // if my_size_num == min_size_num {
    //     println!("I have the shortest size!");
    // }
    // if my_size_num == default_size_num {
    //     println!("Default size suits me!")
    // }

    // Make the code compile. Large should be the default size.
    /*
       Default trait is provided by standard library.
       Has one associated function: default() -> Self
    */
    trait Bounded: Default {
        fn get_max() -> Self;
        fn get_min() -> Self;
    }

    enum Size {
        Small,
        Medium,
        Large,
    }
    // or
    // #[derive(Default)]
    // enum Size {
    //     Small,
    //     Medium,
    //     #[default]
    //     Large,
    // }

    impl Default for Size {
        fn default() -> Self {
            Self::Large
        }
    }
    impl Bounded for Size {
        fn get_max() -> Self {
            Self::Large
        }
        fn get_min() -> Self {
            Self::Small
        }
    }

    fn get_size_num(size: &Size) -> u8 {
        match size {
            Size::Small => 0,
            Size::Medium => 1,
            Size::Large => 2,
        }
    }

    let my_size = Size::Large;
    let min_size_num = get_size_num(&Size::get_min());
    let default_size_num = get_size_num(&Size::default());
    let my_size_num = get_size_num(&my_size);
    if my_size_num == min_size_num {
        println!("I have the shortest size!");
    }
    if my_size_num == default_size_num {
        println!("Default size suits me!")
    }

}

pub fn multiple_supertraits() {
    // Something is missing with the definition of Comparable trait. Fix it.
    // trait Numeric {
    //     fn convert_to_num(&self) -> u8;
    // }
    // 
    // trait Printable {
    //     fn convert_to_str(&self) -> String;
    // }
    // 
    // trait Comparable {
    //     fn print_greater(a: &Self, b: &Self) {
    //         let num1 = a.convert_to_num();
    //         let num2 = b.convert_to_num();
    //         if num1 > num2 {
    //             println!(
    //                 "{} is greater than {}",
    //                 a.convert_to_str(),
    //                 b.convert_to_str()
    //             );
    //         } else if num2 > num1 {
    //             println!(
    //                 "{} is greater than {}",
    //                 b.convert_to_str(),
    //                 a.convert_to_str()
    //             );
    //         } else {
    //             println!("Both sizes are {}", a.convert_to_str());
    //         }
    //     }
    // }
    // 
    // enum Size {
    //     Small,
    //     Medium,
    //     Large,
    // }
    // 
    // impl Numeric for Size {
    //     fn convert_to_num(&self) -> u8 {
    //         match self {
    //             Self::Small => 0,
    //             Self::Medium => 1,
    //             Self::Large => 2,
    //         }
    //     }
    // }
    // 
    // impl Printable for Size {
    //     fn convert_to_str(&self) -> String {
    //         match self {
    //             Self::Small => "Small size".to_string(),
    //             Self::Medium => "Medium size".to_string(),
    //             Self::Large => "Large size".to_string(),
    //         }
    //     }
    // }
    // 
    // impl Comparable for Size {}
    // 
    // let (size1, size2) = (Size::Small, Size::Medium);
    // Comparable::print_greater(&size1, &size2);

    trait Numeric {
        fn convert_to_num(&self) -> u8;
    }

    trait Printable {
        fn convert_to_str(&self) -> String;
    }

    // Something is missing with the definition of Comparable trait. Fix it.
    trait Comparable: Numeric + Printable {
        fn print_greater(a: &Self, b: &Self) {
            let num1 = a.convert_to_num();
            let num2 = b.convert_to_num();
            if num1 > num2 {
                println!(
                    "{} is greater than {}",
                    a.convert_to_str(),
                    b.convert_to_str()
                );
            } else if num2 > num1 {
                println!(
                    "{} is greater than {}",
                    b.convert_to_str(),
                    a.convert_to_str()
                );
            } else {
                println!("Both sizes are {}", a.convert_to_str());
            }
        }
    }

    enum Size {
        Small,
        Medium,
        Large,
    }

    impl Numeric for Size {
        fn convert_to_num(&self) -> u8 {
            match self {
                Self::Small => 0,
                Self::Medium => 1,
                Self::Large => 2,
            }
        }
    }

    impl Printable for Size {
        fn convert_to_str(&self) -> String {
            match self {
                Self::Small => "Small size".to_string(),
                Self::Medium => "Medium size".to_string(),
                Self::Large => "Large size".to_string(),
            }
        }
    }

    impl Comparable for Size {}

    let (size1, size2) = (Size::Small, Size::Medium);
    Comparable::print_greater(&size1, &size2);
}
