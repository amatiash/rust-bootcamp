#![allow(dead_code, unused_variables)]

pub fn code() {
    trait Park {
        fn park(&self);
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

    impl Park for Car {
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

    impl Park for Truck {
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
        let object = create_paintable_object();

        paint_red(&car);
        paint_red(&house);
        paint_red(&object);

        paint_vehicle_red(&car);
    }

    fn paint_red<T: Paint>(object: &T) {
        object.paint("red".to_owned());
    }

    fn paint_vehicle_red<T>(object: &T)
    where
        T: Paint + Park,
    {
        object.paint("red".to_owned());
    }

    fn create_paintable_object() -> impl Paint {
        House {}
    }
}

trait AppendBar {
    fn append_bar(self) -> Self;
}

pub fn implementing_traits_1() {
    // Complete the code
    // trait AppendBar {
    //     fn append_bar(self) -> Self;
    // }
    // impl AppendBar for String {
    //     // Implement `AppendBar` for type `String`.
    // }
    // let s = String::from("Foo");
    // let s = s.append_bar();
    // println!("s: {}", s);
    //
    // #[cfg(test)]
    // mod tests {
    //     use super::*;
    //
    //     #[test]
    //     fn is_foo_bar() {
    //         assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    //     }
    //
    //     #[test]
    //     fn is_bar_bar() {
    //         assert_eq!(
    //             String::from("").append_bar().append_bar(),
    //             String::from("BarBar")
    //         );
    //     }
    // }

    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

impl AppendBar for String {
    fn append_bar(self) -> Self {
        self + "Bar"
    }
}

pub fn implementing_traits_2() {
    // // Complete the code by addressing
    //
    // trait AppendBar {
    //     fn append_bar(self) -> Self;
    // }
    //
    // // Implement trait `AppendBar` for a vector of strings.
    //
    // #[cfg(test)]
    // mod tests {
    //     use super::*;
    //
    //     #[test]
    //     fn is_vec_pop_eq_bar() {
    //         let mut foo = vec![String::from("Foo")].append_bar();
    //         assert_eq!(foo.pop().unwrap(), String::from("Bar"));
    //         assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    //     }
    // }
}

impl AppendBar for Vec<String> {
    fn append_bar(self) -> Self {
        let mut new_vec = self.clone();
        new_vec.push("Bar".to_string());
        new_vec
    }
}

pub fn default_implementations() {
    // Fix this code by updating the Licensed trait.
    // pub trait Licensed {
    //     fn licensing_info(&self) -> String;
    // }
    //
    // struct SomeSoftware {
    //     version_number: i32,
    // }
    //
    // struct OtherSoftware {
    //     version_number: String,
    // }
    //
    // impl Licensed for SomeSoftware {} // Don't edit this line
    // impl Licensed for OtherSoftware {} // Don't edit this line
    //
    // #[cfg(test)]
    // mod tests {
    //     use super::*;
    //
    //     #[test]
    //     fn is_licensing_info_the_same() {
    //         let licensing_info = String::from("Some information");
    //         let some_software = SomeSoftware { version_number: 1 };
    //         let other_software = OtherSoftware {
    //             version_number: "v2.0.0".to_string(),
    //         };
    //         assert_eq!(some_software.licensing_info(), licensing_info);
    //         assert_eq!(other_software.licensing_info(), licensing_info);
    //     }
    // }
}

// Fix this code by updating the Licensed trait.
pub trait Licensed {
    fn licensing_info(&self) -> String {
        String::from("Some information")
    }
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

impl Licensed for SomeSoftware {} // Don't edit this line
impl Licensed for OtherSoftware {} // Don't edit this line

pub fn overriding() {
    // Make the code execute successfully by correctly implementing Message trait for Cat type.
    // trait Message {
    //     fn message(&self) -> String {
    //         "Default Message!".to_string()
    //     }
    // }
    //
    // struct Fish;
    // struct Cat;
    // impl Message for Fish {}
    // impl Message for Cat {}
    //
    // let fish = Fish;
    // let cat = Cat;
    // assert_eq!(String::from("Default Message!"), fish.message());
    // assert_eq!(String::from("Meow 🐱"), cat.message());

    trait Message {
        fn message(&self) -> String {
            "Default Message!".to_string()
        }
    }

    struct Fish;
    struct Cat;
    impl Message for Fish {}
    impl Message for Cat {
        fn message(&self) -> String {
            "Meow 🐱".to_string()
        }
    }

    let fish = Fish;
    let cat = Cat;
    assert_eq!(String::from("Default Message!"), fish.message());
    assert_eq!(String::from("Meow 🐱"), cat.message());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = String::from("Some information");
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
