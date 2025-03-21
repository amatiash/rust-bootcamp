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

pub fn specifying_trait_bounds_1() {
    // Complete the function signatures.
    // trait Message {
    //     fn message(&self) -> String {
    //         "I love Rust 🦀".to_string()
    //     }
    // }
    //
    // fn print_msg1<T:>(input: &T) {
    //     println!("{}", input.message());
    // }
    //
    // fn print_msg2(input:) {
    //     println!("{}", input.message());
    // }
    //
    // fn print_msg3<T>(input: &T)
    // where
    // {
    //     println!("{}", input.message());
    // }
    //
    // struct Dummy;
    //
    // impl Message for Dummy {}
    //
    // let var = Dummy;
    // print_msg1(&var);
    // print_msg2(&var);
    // print_msg3(&var);

    trait Message {
        fn message(&self) -> String {
            "I love Rust 🦀".to_string()
        }
    }

    fn print_msg1<T: Message>(input: &T) {
        println!("{}", input.message());
    }

    fn print_msg2(input: &impl Message) {
        println!("{}", input.message());
    }

    fn print_msg3<T>(input: &T)
    where
        T: Message,
    {
        println!("{}", input.message());
    }

    struct Dummy;

    impl Message for Dummy {}

    let var = Dummy;
    print_msg1(&var);
    print_msg2(&var);
    print_msg3(&var);
}

pub fn specifying_trait_bounds_2() {
    // Make the code compile by completing the function signature of `print_message`.
    // trait Message {
    //     fn message(&self) -> String {
    //         "How are you?".to_string()
    //     }
    // }
    //
    // trait Printer {
    //     fn print(&self, printable: &impl Message) {
    //         println!("Message is: {}", printable.message());
    //     }
    // }
    //
    // struct M;
    // struct P;
    //
    // impl Message for M {}
    // impl Printer for P {}
    //
    // fn print_message<T, U>(msg, printer)
    // where
    // {
    //     printer.print(msg);
    // }
    //
    // let m = M;
    // let p = P;
    // print_message(&m, &p);

    trait Message {
        fn message(&self) -> String {
            "How are you?".to_string()
        }
    }

    trait Printer {
        fn print(&self, printable: &impl Message) {
            println!("Message is: {}", printable.message());
        }
    }

    struct M;
    struct P;

    impl Message for M {}
    impl Printer for P {}

    fn print_message<T, U>(msg: &T, printer: &U)
    where
        U: Printer,
        T: Message,
    {
        printer.print(msg);
    }

    let m = M;
    let p = P;
    print_message(&m, &p);
}

pub fn specifying_trait_bounds_3() {
    // Complete the code so that it compiles.
    // pub trait Licensed {
    //     fn licensing_info(&self) -> String {
    //         "some information".to_string()
    //     }
    // }
    //
    // struct SomeSoftware {}
    //
    // struct OtherSoftware {}
    //
    // impl Licensed for SomeSoftware {}
    // impl Licensed for OtherSoftware {}
    //
    // // YOU MAY ONLY CHANGE THE NEXT LINE
    // fn compare_license_types(software: ??, software_two: ??) -> bool {
    //     software.licensing_info() == software_two.licensing_info()
    // }
}

pub trait Licensed1 {
    fn licensing_info(&self) -> String {
        "some information".to_string()
    }
}

struct SomeSoftware1 {}

struct OtherSoftware1 {}

impl Licensed1 for SomeSoftware1 {}
impl Licensed1 for OtherSoftware1 {}

// YOU MAY ONLY CHANGE THE NEXT LINE
fn compare_license_types(software: impl Licensed1, software_two: impl Licensed1) -> bool {
    software.licensing_info() == software_two.licensing_info()
}

pub fn multiple_trait_bounds_1() {
    // Make the code compile by completing the function signature of `get_double_str`.
    // trait Double {
    //     fn double(&self) -> Self;
    // }
    //
    // trait Printable {
    //     fn convert_to_str(self) -> String;
    // }
    //
    // fn get_double_str(input) -> String
    // {
    //     let doubled = input.double();
    //     doubled.convert_to_str()
    // }
    //
    // impl Double for i32 {
    //     fn double(&self) -> Self {
    //         2 * self
    //     }
    // }
    //
    // impl Printable for i32 {
    //     fn convert_to_str(self) -> String {
    //         format!("{self}")
    //     }
    // }
    //
    // let num = 22;
    // let mut msg = format!("{num} doubled is ");
    // msg.push_str(&get_double_str(num));
    // println!("{msg}");

    trait Double {
        fn double(&self) -> Self;
    }

    trait Printable {
        fn convert_to_str(self) -> String;
    }

    impl Double for i32 {
        fn double(&self) -> Self {
            2 * self
        }
    }

    impl Printable for i32 {
        fn convert_to_str(self) -> String {
            format!("{self}")
        }
    }

    fn get_double_str<T>(input: T) -> String // Can also be: fn get_double_str(input: impl Double+Printable) -> String
    where
        T: Double + Printable,
    {
        let doubled = input.double();
        doubled.convert_to_str()
    }

    let num = 22;
    let mut msg = format!("{num} doubled is ");
    msg.push_str(&get_double_str(num));
    println!("{msg}");
}

pub fn multiple_trait_bounds_2() {
    // Complete the code so that it compiles.
    // pub trait SomeTrait {
    //     fn some_function(&self) -> bool {
    //         true
    //     }
    // }
    //
    // pub trait OtherTrait {
    //     fn other_function(&self) -> bool {
    //         true
    //     }
    // }
    //
    // struct SomeStruct {}
    // struct OtherStruct {}
    //
    // impl SomeTrait for SomeStruct {}
    // impl OtherTrait for SomeStruct {}
    // impl SomeTrait for OtherStruct {}
    // impl OtherTrait for OtherStruct {}
    //
    // // YOU MAY ONLY CHANGE THE NEXT LINE
    // fn some_func(item: ??) -> bool {
    //     item.some_function() && item.other_function()
    // }
    //
    // some_func(SomeStruct {});
    // some_func(OtherStruct {});

    pub trait SomeTrait {
        fn some_function(&self) -> bool {
            true
        }
    }

    pub trait OtherTrait {
        fn other_function(&self) -> bool {
            true
        }
    }

    struct SomeStruct {}
    struct OtherStruct {}

    impl SomeTrait for SomeStruct {}
    impl OtherTrait for SomeStruct {}
    impl SomeTrait for OtherStruct {}
    impl OtherTrait for OtherStruct {}

    // YOU MAY ONLY CHANGE THE NEXT LINE
    fn some_func<T>(item: T) -> bool
    where
        T: SomeTrait + OtherTrait,
    {
        item.some_function() && item.other_function()
    }

    some_func(SomeStruct {});
    some_func(OtherStruct {});
}

pub fn returning_trait_bounds() {
    // Complete function signature of `get_bigger`.
    // Only Addable trait's functions should be callable on its return value.
    // trait Addable {
    //     fn add_one(&self) -> Self;
    //     fn are_equal(a: &Self, b: &Self) -> bool;
    // }
    //
    // impl Addable for u8 {
    //     fn add_one(&self) -> Self {
    //         if *self == u8::MAX {
    //             *self
    //         } else {
    //             self + 1
    //         }
    //     }
    //     fn are_equal(a: &Self, b: &Self) -> bool {
    //         a == b
    //     }
    // }
    //
    // fn get_bigger(a: u8, b: u8) -> {
    //     if a > b {
    //         a
    //     } else {
    //         b
    //     }
    // }
    //
    // let (num1, num2) = (125, 220);
    // let bigger = get_bigger(num1, num2);
    // if Addable::are_equal(&bigger, &bigger.add_one()) {
    //     println!("Bigger number has max value")
    // } else {
    //     println!("Both numbers are smaller than max value");
    // }

    trait Addable {
        fn add_one(&self) -> Self;
        fn are_equal(a: &Self, b: &Self) -> bool;
    }

    impl Addable for u8 {
        fn add_one(&self) -> Self {
            if *self == u8::MAX { *self } else { self + 1 }
        }
        fn are_equal(a: &Self, b: &Self) -> bool {
            a == b
        }
    }

    fn get_bigger(a: u8, b: u8) -> impl Addable {
        if a > b { a } else { b }
    }

    let (num1, num2) = (125, 220);
    let bigger = get_bigger(num1, num2);
    if Addable::are_equal(&bigger, &bigger.add_one()) {
        println!("Bigger number has max value")
    } else {
        println!("Both numbers are smaller than max value");
    }
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

    #[test]
    fn compare_license_information() {
        let some_software = SomeSoftware1 {};
        let other_software = OtherSoftware1 {};

        assert!(compare_license_types(some_software, other_software));
    }

    #[test]
    fn compare_license_information_backwards() {
        let some_software = SomeSoftware1 {};
        let other_software = OtherSoftware1 {};

        assert!(compare_license_types(other_software, some_software));
    }
}
