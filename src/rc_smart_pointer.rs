#![allow(dead_code, unused_variables)]

pub fn code() {
    use std::rc::Rc;

    struct Database {}

    struct AuthService {
        db: Rc<Database>,
    }

    struct ContentService {
        db: Rc<Database>,
    }

    let db = Rc::new(Database {});
    let auth_service = AuthService { db: db.clone() };
    let content_service = ContentService { db: db.clone() };
}

pub fn counting_refs_1() {
    // We can get the number of references behind a Rc value using Rc::strong_count.
    // Can you guess the number of references alive at various places?
    // Only use numbers as second parameter to assert_eq's.
    // use std::rc::Rc;
    // let ptr1 = Rc::new('🦀'); // A crab stored on the heap
    // assert_eq!(Rc::strong_count(&ptr1),);
    // {
    //     let ptr2 = Rc::clone(&ptr1);
    //     assert_eq!(Rc::strong_count(&ptr1),);
    //     {
    //         let ptr3 = Rc::clone(&ptr2);
    //         assert_eq!(Rc::strong_count(&ptr1),);
    //     }
    //     assert_eq!(Rc::strong_count(&ptr1),);
    // }
    // assert_eq!(Rc::strong_count(&ptr1),);

    // We can get the number of references behind a Rc value using Rc::strong_count.
    // Can you guess the number of references alive at various places?
    // Only use numbers as second parameter to assert_eq's.
    use std::rc::Rc;
    let ptr1 = Rc::new('🦀'); // A crab stored on the heap
    assert_eq!(Rc::strong_count(&ptr1), 1);
    {
        let ptr2 = Rc::clone(&ptr1);
        assert_eq!(Rc::strong_count(&ptr1), 2);
        {
            let ptr3 = Rc::clone(&ptr2);
            assert_eq!(Rc::strong_count(&ptr1), 3);
        }
        assert_eq!(Rc::strong_count(&ptr1), 2);
    }
    assert_eq!(Rc::strong_count(&ptr1), 1);
}

pub fn counting_refs_2() {
    // In this exercise, we want to express the concept of multiple owners via the Rc<T> type.
    // This is a model of our solar system - there is a Sun type and multiple Planets.
    // The Planets take ownership of the sun, indicating that they revolve around the sun.
    // Make this code compile by using the proper Rc primitives to express that the sun has multiple owners.
    // use std::rc::Rc;
    //
    // #[derive(Debug)]
    // struct Sun {}
    //
    // #[derive(Debug)]
    // enum Planet {
    //     Mercury(Rc<Sun>),
    //     Venus(Rc<Sun>),
    //     Earth(Rc<Sun>),
    //     Mars(Rc<Sun>),
    //     Jupiter(Rc<Sun>),
    //     Saturn(Rc<Sun>),
    //     Uranus(Rc<Sun>),
    //     Neptune(Rc<Sun>),
    // }
    //
    // impl Planet {
    //     fn details(&self) {
    //         println!("Hi from {:?}!", self)
    //     }
    // }
    //
    // let sun = Rc::new(Sun {});
    // println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference
    //
    // let mercury = Planet::Mercury(Rc::clone(&sun));
    // println!("reference count = {}", Rc::strong_count(&sun)); // 2 references
    // mercury.details();
    //
    // let venus = Planet::Venus(Rc::clone(&sun));
    // println!("reference count = {}", Rc::strong_count(&sun)); // 3 references
    // venus.details();
    //
    // let earth = Planet::Earth(Rc::clone(&sun));
    // println!("reference count = {}", Rc::strong_count(&sun)); // 4 references
    // earth.details();
    //
    // let mars = Planet::Mars(Rc::clone(&sun));
    // println!("reference count = {}", Rc::strong_count(&sun)); // 5 references
    // mars.details();
    //
    // let jupiter = Planet::Jupiter(Rc::clone(&sun));
    // println!("reference count = {}", Rc::strong_count(&sun)); // 6 references
    // jupiter.details();
    //
    // // To do
    // let saturn = Planet::Saturn(Rc::new(Sun {}));
    // println!("reference count = {}", Rc::strong_count(&sun)); // 7 references
    // saturn.details();
    //
    // // To do
    // let uranus = Planet::Uranus(Rc::new(Sun {}));
    // println!("reference count = {}", Rc::strong_count(&sun)); // 8 references
    // uranus.details();
    //
    // // To do
    // let neptune = Planet::Neptune(Rc::new(Sun {}));
    // println!("reference count = {}", Rc::strong_count(&sun)); // 9 references
    // neptune.details();
    //
    // assert_eq!(Rc::strong_count(&sun), 9);
    //
    // drop(neptune);
    // println!("reference count = {}", Rc::strong_count(&sun)); // 8 references
    //
    // drop(uranus);
    // println!("reference count = {}", Rc::strong_count(&sun)); // 7 references
    //
    // drop(saturn);
    // println!("reference count = {}", Rc::strong_count(&sun)); // 6 references
    //
    // drop(jupiter);
    // println!("reference count = {}", Rc::strong_count(&sun)); // 5 references
    //
    // drop(mars);
    // println!("reference count = {}", Rc::strong_count(&sun)); // 4 references
    //
    // // To do
    // println!("reference count = {}", Rc::strong_count(&sun)); // 3 references
    //
    // // To do
    // println!("reference count = {}", Rc::strong_count(&sun)); // 2 references
    //
    // // To do
    // println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference
    //
    // assert_eq!(Rc::strong_count(&sun), 1);

    // In this exercise, we want to express the concept of multiple owners via the Rc<T> type.
    // This is a model of our solar system - there is a Sun type and multiple Planets.
    // The Planets take ownership of the sun, indicating that they revolve around the sun.
    // Make this code compile by using the proper Rc primitives to express that the sun has multiple owners.

    use std::rc::Rc;

    #[derive(Debug)]
    struct Sun {}

    #[derive(Debug)]
    enum Planet {
        Mercury(Rc<Sun>),
        Venus(Rc<Sun>),
        Earth(Rc<Sun>),
        Mars(Rc<Sun>),
        Jupiter(Rc<Sun>),
        Saturn(Rc<Sun>),
        Uranus(Rc<Sun>),
        Neptune(Rc<Sun>),
    }

    impl Planet {
        fn details(&self) {
            println!("Hi from {:?}!", self)
        }
    }

    let sun = Rc::new(Sun {});
    println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference

    let mercury = Planet::Mercury(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 2 references
    mercury.details();

    let venus = Planet::Venus(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 3 references
    venus.details();

    let earth = Planet::Earth(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 4 references
    earth.details();

    let mars = Planet::Mars(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 5 references
    mars.details();

    let jupiter = Planet::Jupiter(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 6 references
    jupiter.details();

    let saturn = Planet::Saturn(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 7 references
    saturn.details();

    let uranus = Planet::Uranus(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 8 references
    uranus.details();

    let neptune = Planet::Neptune(Rc::clone(&sun));
    println!("reference count = {}", Rc::strong_count(&sun)); // 9 references
    neptune.details();

    assert_eq!(Rc::strong_count(&sun), 9);

    drop(neptune);
    println!("reference count = {}", Rc::strong_count(&sun)); // 8 references

    drop(uranus);
    println!("reference count = {}", Rc::strong_count(&sun)); // 7 references

    drop(saturn);
    println!("reference count = {}", Rc::strong_count(&sun)); // 6 references

    drop(jupiter);
    println!("reference count = {}", Rc::strong_count(&sun)); // 5 references

    drop(mars);
    println!("reference count = {}", Rc::strong_count(&sun)); // 4 references

    drop(earth);
    println!("reference count = {}", Rc::strong_count(&sun)); // 3 references

    drop(venus);
    println!("reference count = {}", Rc::strong_count(&sun)); // 2 references

    drop(mercury);
    println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference

    assert_eq!(Rc::strong_count(&sun), 1);
}
