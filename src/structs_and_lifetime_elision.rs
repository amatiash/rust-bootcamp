#![allow(dead_code, unused_variables)]

pub fn code() {
    struct Tweet<'a> {
        content: &'a str,
    }

    // 1. Each parameter that is a reference gets its own lifetime parameter.
    // 2. If there is exactly one input lifetime parameter, that lifetime
    //    is assigned to all output lifetime parameters.
    // 3. If there are multiple input lifetime parameters, but one of them is
    //    &self or &mut self, the lifetime of self is assigned to all output
    //    lifetime parameters.

    impl<'a> Tweet<'a> {
        fn replace_content(&mut self, content: &'a str) -> &str {
            let old_content = self.content;
            self.content = content;
            old_content
        }
    }

    let mut tweet = Tweet { content: "example" };
    let old_content = tweet.replace_content("replace_example");
    println!("{old_content}");
    println!("{}", tweet.content);
    fn take_and_return_content<'a>(content: &'a str, content2: &'a str) -> &'a str {
        content
    }
}

pub fn lifetimes_in_structs() {
    // Something is missing from our struct definition. Can you fix it?
    // struct Book {
    //     author: &str,
    //     title: &str,
    // }
    //
    // let name = String::from("Jill Smith");
    // let title = String::from("Fish Flying");
    // let book = Book {
    //     author: &name,
    //     title: &title,
    // };
    //
    // println!("{} by {}", book.title, book.author);

    struct Book<'a> {
        author: &'a str,
        title: &'a str,
    }

    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book {
        author: &name,
        title: &title,
    };

    println!("{} by {}", book.title, book.author);
}

pub fn lifetime_elision() {
    // The code below executes successfully. However, remove the lifetimes from wherever they can be inferred implicitly.
    // struct NameStack<'a> {
    //     names: Vec<&'a str>,
    // }
    //
    // impl<'a> NameStack<'a> {
    //     fn new() -> Self {
    //         NameStack { names: Vec::new() }
    //     }
    //     fn add_name(&mut self, name: &'a str) {
    //         self.names.push(name);
    //     }
    //     fn remove_name_with_substr<'b>(&mut self, sub_str: &'b str) -> &'a str {
    //         for i in 0..self.names.len() {
    //             if self.names[i].contains(sub_str) {
    //                 let removed = self.names.remove(i);
    //                 return removed;
    //             }
    //         }
    //         panic!("Name with substring not found");
    //     }
    // }
    //
    // let mut my_names = NameStack::new();
    // my_names.add_name("Alice");
    // my_names.add_name("Bob");
    // my_names.add_name("Cindy");
    // my_names.add_name("Emily");
    // let removed = my_names.remove_name_with_substr("ice");
    // println!("Removed: {removed}");
    // assert_eq!(my_names.names.len(), 3);

    struct NameStack<'a> {
        names: Vec<&'a str>,
    }

    impl<'a> NameStack<'a> {
        fn new() -> Self {
            NameStack { names: Vec::new() }
        }
        fn add_name(&mut self, name: &'a str) {
            self.names.push(name);
        }
        fn remove_name_with_substr(&mut self, sub_str: &str) -> &str {
            for i in 0..self.names.len() {
                if self.names[i].contains(sub_str) {
                    let removed = self.names.remove(i);
                    return removed;
                }
            }
            panic!("Name with substring not found");
        }
    }

    let mut my_names = NameStack::new();
    my_names.add_name("Alice");
    my_names.add_name("Bob");
    my_names.add_name("Cindy");
    my_names.add_name("Emily");
    let removed = my_names.remove_name_with_substr("ice");
    println!("Removed: {removed}");
    assert_eq!(my_names.names.len(), 3);
}
