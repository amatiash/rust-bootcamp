#![allow(dead_code, unused_variables)]

pub fn code() {
    trait Iterator {
        type Item;

        fn next(&mut self) -> Option<Self::Item>;
    }

    trait IntoIterator {
        type Item;
        type IntoIter: Iterator;
        fn into_iter(self) -> Self::IntoIter;
    }

    struct MyStruct {}

    impl Iterator for MyStruct {
        type Item = String;

        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let mut m = MyStruct {};
    let item = m.next();
}

pub fn iterator_trait() {
    // Make the code compile by implementing the Iterator trait for `Queue`.
    // struct Queue {
    //     items: Vec<i32>,
    // }
    // 
    // impl Queue {
    //     fn new(items: Vec<i32>) -> Self {
    //         Self { items }
    //     }
    // }
    // 
    // impl Iterator for Queue {
    //     type Item;
    //     fn next(&mut self) -> Option<Self::Item> {}
    // }
    // 
    // let mut queue = Queue::new(vec![3, 2, 1]);
    // assert!(matches!(queue.next(), Some(3)));
    // assert!(matches!(queue.next(), Some(2)));
    // assert!(matches!(queue.next(), Some(1)));
    // assert!(matches!(queue.next(), None));

    struct Queue {
        items: Vec<i32>,
    }

    impl Queue {
        fn new(items: Vec<i32>) -> Self {
            Self { items }
        }
    }

    impl Iterator for Queue {
        type Item = i32;
        fn next(&mut self) -> Option<Self::Item> {
            if self.items.is_empty() {
                None
            } else {
                Some(self.items.remove(0))
            }
        }
    }

    let mut queue = Queue::new(vec![3, 2, 1]);
    assert!(matches!(queue.next(), Some(3)));
    assert!(matches!(queue.next(), Some(2)));
    assert!(matches!(queue.next(), Some(1)));
    assert!(matches!(queue.next(), None));
}
