#![allow(dead_code, unused_variables)]

pub fn code() {
    use std::ops::{Deref, DerefMut};

    struct MySmartPointer<T> {
        value: T
    }

    impl<T> MySmartPointer<T> {
        fn new(value: T) -> MySmartPointer<T> {
            MySmartPointer { value }
        }
    }

    impl<T> Deref for MySmartPointer<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.value
        }
    }

    impl<T> DerefMut for MySmartPointer<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.value
        }
    }

    fn print(s: &str) {
        println!("{s}");
    }

    let mut s = MySmartPointer::new(
        Box::new("Let's Get Rusty".to_owned())
    );
    // let s = &(***s);

    // &MySmartPointer -> &Box -> &String -> &str
    print(&mut s);
}

pub fn example() {
    // Complete the code
    // use std::rc::Rc;
    // 
    // struct Employee {
    //     name: String,
    //     id: u32,
    // }
    // 
    // impl Employee {
    //     fn new(name: &str, id: u32) -> Self {
    //         Employee {
    //             name: name.to_string(),
    //             id,
    //         }
    //     }
    //     fn print_details(&self) {
    //         println!("Name: {}, ID: {}", self.name, self.id);
    //     }
    // }
    // 
    // let emp1 = Box::new(Employee::new("Alice", 1234));
    // // Call print_details on emp1
    // let emp2 = Box::new(emp1);
    // // Call print_details on emp2
    // let emp3 = Rc::new(emp2);
    // // Call print_details on emp3

    use std::rc::Rc;

    struct Employee {
        name: String,
        id: u32,
    }

    impl Employee {
        fn new(name: &str, id: u32) -> Self {
            Employee {
                name: name.to_string(),
                id,
            }
        }
        fn print_details(&self) {
            println!("Name: {}, ID: {}", self.name, self.id);
        }
    }

    let emp1 = Box::new(Employee::new("Alice", 1234));
    emp1.print_details();
    let emp2 = Box::new(emp1);
    emp2.print_details();
    let emp3 = Rc::new(emp2);
    emp3.print_details();
}

pub fn deref_and_deref_mut() {
    // Make the code compile by implementing Deref & DerefMut for Wrapper.
    // use std::ops::{Deref, DerefMut};
    // 
    // struct Wrapper<T>(T);
    // 
    // impl<T> Deref for Wrapper<T> {
    //     type Target;
    //     fn deref(&self) -> &Self::Target {}
    // }
    // 
    // impl<T> DerefMut for Wrapper<T> {
    //     fn deref_mut(&mut self) -> &mut Self::Target {}
    // }
    // 
    // fn are_equal(a: &str, b: &str) -> bool {
    //     a == b
    // }
    // 
    // let mut my_str = Wrapper(String::from("Ferris"));
    // my_str.push_str(" the crab!!");
    // my_str.pop();
    // assert!(are_equal(&my_str, "Ferris the crab!"));

    // Make the code compile by implementing Deref & DerefMut for Wrapper.
    use std::ops::{Deref, DerefMut};

    struct Wrapper<T>(T);

    impl<T> Deref for Wrapper<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<T> DerefMut for Wrapper<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }

    fn are_equal(a: &str, b: &str) -> bool {
        a == b
    }

    let mut my_str = Wrapper(String::from("Ferris"));
    my_str.push_str(" the crab!!");
    my_str.pop();
    assert!(are_equal(&my_str, "Ferris the crab!"));
}

pub fn shared_ownership_with_mutability() {
    // // Complete the update_value function.
    // use core::fmt::Debug;
    // use std::cell::RefCell;
    // use std::rc::Rc;
    // 
    // fn update_value<T>(owner: &Rc<RefCell<T>>, value: T) {}
    // 
    // fn print_value<T: Debug>(owner: &Rc<RefCell<T>>) {
    //     println!("{:?}", owner.borrow());
    // }
    // 
    // let owner1 = Rc::new(RefCell::new("Harry"));
    // print_value(&owner1);
    // let owner2 = Rc::clone(&owner1);
    // update_value(&owner2, "Ron");
    // print_value(&owner1);
    // print_value(&owner2);

    // Complete the update_value function.
    use core::fmt::Debug;
    use std::cell::RefCell;
    use std::rc::Rc;

    fn update_value<T>(owner: &Rc<RefCell<T>>, value: T) {
        *owner.borrow_mut() = value;
        // owner.replace(value);
    }

    fn print_value<T: Debug>(owner: &Rc<RefCell<T>>) {
        println!("{:?}", owner.borrow());
    }

    let owner1 = Rc::new(RefCell::new("Harry"));
    print_value(&owner1);
    let owner2 = Rc::clone(&owner1);
    update_value(&owner2, "Ron");
    print_value(&owner1);
    print_value(&owner2);
}
