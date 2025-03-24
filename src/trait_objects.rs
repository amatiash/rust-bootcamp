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
        year: u16,
    }

    struct Car {
        info: VehicleInfo,
    }

    impl Vehicle for Car {
        fn park(&self) {
            println!("parking car!");
        }
    }

    impl Paint for Car {}

    struct Truck {
        info: VehicleInfo,
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

    fn main() {
        let car = Car {
            info: VehicleInfo {
                make: "Honda".to_owned(),
                model: "Civic".to_owned(),
                year: 1995,
            },
        };
        let house = House {};
        let object = create_paintable_object(true);

        let paintable_objects: Vec<&dyn Paint> = vec![&car, &house];

        paint_red(&car);
        paint_red(&house);
        paint_red(object.as_ref());

        paint_vehicle_red(&car);
    }

    fn paint_red(object: &dyn Paint) {
        object.paint("red".to_owned());
    }

    fn paint_vehicle_red<T>(object: &T)
    where
        T: Vehicle,
    {
        object.paint("red".to_owned());
    }

    fn create_paintable_object(vehicle: bool) -> Box<dyn Paint> {
        if vehicle {
            Box::new(Car {
                info: VehicleInfo {
                    make: "Honda".to_owned(),
                    model: "Civic".to_owned(),
                    year: 1995,
                },
            })
        } else {
            Box::new(House {})
        }
    }
}

pub fn returning_and_passing_to_functions() {
    // Make the code compile by completing the function signatures.
    // trait Shape {
    //     fn shape(&self) -> String;
    // }
    //
    // struct Triangle;
    //
    // struct Square;
    //
    // impl Shape for Triangle {
    //     fn shape(&self) -> String {
    //         "🔺".to_string()
    //     }
    // }
    //
    // impl Shape for Square {
    //     fn shape(&self) -> String {
    //         "🟥".to_string()
    //     }
    // }
    //
    // fn get_shape(side_count: u8) ->  {
    //     match side_count {
    //         3 => Box::new(Triangle),
    //         4 => Box::new(Square),
    //         _ => panic!("No shape with side count available"),
    //     }
    // }
    //
    // fn draw_shape(to_draw: &) {
    //     println!("{}", to_draw.shape())
    // }
    //
    // let my_shape = get_shape(4);
    // draw_shape(my_shape.as_ref());

    trait Shape {
        fn shape(&self) -> String;
    }

    struct Triangle;

    struct Square;

    impl Shape for Triangle {
        fn shape(&self) -> String {
            "🔺".to_string()
        }
    }

    impl Shape for Square {
        fn shape(&self) -> String {
            "🟥".to_string()
        }
    }

    fn get_shape(side_count: u8) -> Box<dyn Shape> {
        match side_count {
            3 => Box::new(Triangle),
            4 => Box::new(Square),
            _ => panic!("No shape with side count available"),
        }
    }

    fn draw_shape(to_draw: &dyn Shape) {
        println!("{}", to_draw.shape())
    }

    let my_shape = get_shape(4);
    draw_shape(my_shape.as_ref());
}

pub fn trait_objects_vectors() {
    // Make the code compile by annotating the type for the vector.
    // trait Shape {
    //     fn shape(&self) -> String;
    //     fn side_count(&self) -> u8;
    // }
    //
    // struct Triangle;
    //
    // struct Square;
    //
    // impl Shape for Triangle {
    //     fn shape(&self) -> String {
    //         "🔺".to_string()
    //     }
    //     fn side_count(&self) -> u8 {
    //         3
    //     }
    // }
    //
    // impl Shape for Square {
    //     fn shape(&self) -> String {
    //         "🟥".to_string()
    //     }
    //     fn side_count(&self) -> u8 {
    //         4
    //     }
    // }
    //
    // let shape1 = Square;
    // let shape2 = Square;
    // let shape3 = Triangle;
    // let shapes = vec![&shape1, &shape2, &shape3];
    //
    // // fetch the first triangle and print it
    // for shape in shapes {
    //     if shape.side_count() == 3 {
    //         println!("{}", shape.shape());
    //         break;
    //     }
    // }

    trait Shape {
        fn shape(&self) -> String;
        fn side_count(&self) -> u8;
    }

    struct Triangle;

    struct Square;

    impl Shape for Triangle {
        fn shape(&self) -> String {
            "🔺".to_string()
        }
        fn side_count(&self) -> u8 {
            3
        }
    }

    impl Shape for Square {
        fn shape(&self) -> String {
            "🟥".to_string()
        }
        fn side_count(&self) -> u8 {
            4
        }
    }

    let shape1 = Square;
    let shape2 = Square;
    let shape3 = Triangle;
    let shapes: Vec<&dyn Shape> = vec![&shape1, &shape2, &shape3];

    // fetch the first triangle and print it
    for shape in shapes {
        if shape.side_count() == 3 {
            println!("{}", shape.shape());
            break;
        }
    }
}
