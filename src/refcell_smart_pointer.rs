#![allow(dead_code, unused_variables)]

pub fn code() {
    use std::rc::Rc;
    use std::cell::RefCell;

    struct Database {
        max_connections: u32
    }

    struct AuthService {
        db: Rc<RefCell<Database>>
    }

    struct ContentService {
        db: Rc<RefCell<Database>>
    }

    let db = Rc::new(RefCell::new(Database {
        max_connections: 100
    }));
    let auth_service = AuthService { db: Rc::clone(&db) };
    let content_service = ContentService { db: Rc::clone(&db) };

    let mut r1 = db.borrow_mut();
    r1.max_connections = 200;
}

pub fn borrowing() {
    // Complete the following code.
    // use std::cell::RefCell;
    //
    // // storing value 5 on heap
    // let ptr = RefCell::new(5);
    // // get an immutable reference to the stored value
    // let ref1 = ;
    // println!("Stored value: {}", ref1);
    // drop(ref1);
    // // get a mutable reference to the stored value
    // let mut ref2 = ;
    // *ref2 = 6; // Note: we can mutate the value associated with ptr, even though it is not marked as mut
    // println!("Stored value: {}", ref2);

    // Complete the following code.
    use std::cell::RefCell;

    // storing value 5 on heap
    let ptr = RefCell::new(5);
    // get an immutable reference to the stored value
    let ref1 = ptr.borrow();
    println!("Stored value: {}", ref1);
    drop(ref1);
    // get a mutable reference to the stored value
    let mut ref2 = ptr.borrow_mut();
    *ref2 = 6; // Note: we can mutate the value associated with ptr, even though it is not marked as mut
    println!("Stored value: {}", ref2);

}

pub fn interior_mutability() {
    // // Fix the print_details method. You can only modify the method body.
    // use std::cell::RefCell;
    //
    // struct Student {
    //     name: String,
    //     marks: u8,
    //     grade: RefCell<char>,
    // }
    //
    // impl Student {
    //     fn new(name: &str, marks: u8) -> Self {
    //         Student {
    //             name: name.to_owned(),
    //             marks,
    //             grade: RefCell::new('X'),
    //         }
    //     }
    //
    //     fn print_details(&self) {
    //         let grade = match self.marks {
    //             0..=33 => 'C',
    //             34..=60 => 'B',
    //             _ => 'A',
    //         };
    //         self.grade = grade;
    //         println!(
    //             "name: {}, marks: {}, grade: {}",
    //             self.name,
    //             self.marks,
    //             self.grade.borrow()
    //         );
    //     }
    // }
    //
    // let student = Student::new("Harry", 70);
    // student.print_details();

    // Fix the print_details method. You can only modify the method body.
    use std::cell::RefCell;

    struct Student {
        name: String,
        marks: u8,
        grade: RefCell<char>,
    }

    impl Student {
        fn new(name: &str, marks: u8) -> Self {
            Student {
                name: name.to_owned(),
                marks,
                grade: RefCell::new('X'),
            }
        }

        fn print_details(&self) {
            let grade = match self.marks {
                0..=33 => 'C',
                34..=60 => 'B',
                _ => 'A',
            };
            *self.grade.borrow_mut() = grade;
            println!(
                "name: {}, marks: {}, grade: {}",
                self.name,
                self.marks,
                self.grade.borrow()
            );
        }
    }

    let student = Student::new("Harry", 70);
    student.print_details();
}
