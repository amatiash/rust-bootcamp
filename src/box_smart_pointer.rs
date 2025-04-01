#![allow(dead_code, unused_variables)]

pub fn code() {
    trait UIComponent {
        fn render(&self) {
            println!("Rendering component...");
        }
    }

    struct Button {
        text: String,
    }

    impl UIComponent for Button {}

    struct Container {
        name: String,
        child: Box<Container>,
    }

    impl UIComponent for Container {}

    let button_a = Button {
        text: "button a".to_owned(),
    };
    let button_b = Box::new(Button {
        text: "button b".to_owned(),
    });

    let button_c = button_a;
    let button_d = button_b;

    let components: Vec<Box<dyn UIComponent>> = vec![Box::new(button_c), button_d];
}

pub fn creation() {
    // Initialize heap_var to store value 4 on the heap & make the code execute successfully.
    // let stack_var = 5;
    // // Initialize this variable
    // let heap_var
    // let res = stack_var + *heap_var;
    // assert_eq!(res, 9);

    let stack_var = 5;
    let heap_var = Box::new(4);
    let res = stack_var + *heap_var;
    assert_eq!(res, 9);
}

pub fn recursive_types() {
    // The recursive type we're implementing in this exercise is the `cons list` - a data structure
    // frequently found in functional programming languages. Each item in a cons list contains two
    // elements: the value of the current item and the next item. The last item is a value called `Nil`.
    //
    // Step 1: use a `Box` in the enum definition to make the code compile
    // Step 2: create both empty and non-empty cons lists

    // #[derive(PartialEq, Debug)]
    // pub enum List {
    //     Cons(i32, List),
    //     Nil,
    // }
    //
    // println!("This is an empty cons list: {:?}", create_empty_list());
    // println!(
    //     "This is a non-empty cons list: {:?}",
    //     create_non_empty_list()
    // );
    //
    // pub fn create_empty_list() -> List {
    //     todo!()
    // }
    //
    // pub fn create_non_empty_list() -> List {
    //     todo!()
    // }
    //
    // #[cfg(test)]
    // mod tests {
    //     use super::*;
    //
    //     #[test]
    //     fn test_create_empty_list() {
    //         assert_eq!(List::Nil, create_empty_list())
    //     }
    //
    //     #[test]
    //     fn test_create_non_empty_list() {
    //         assert_ne!(create_empty_list(), create_non_empty_list())
    //     }
    // }

    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

// The recursive type we're implementing in this exercise is the `cons list` - a data structure
// frequently found in functional programming languages. Each item in a cons list contains two
// elements: the value of the current item and the next item. The last item is a value called `Nil`.
//
// Step 1: use a `Box` in the enum definition to make the code compile
// Step 2: create both empty and non-empty cons lists

#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub fn create_empty_list() -> List {
    List::Nil
}

pub fn create_non_empty_list() -> List {
    List::Cons(1, Box::new(List::Nil))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
