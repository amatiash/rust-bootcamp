pub fn visibility() {
    // Something's missing. Fix the code so that it compiles.
    // mod sausage_factory {
    //     // Don't let anybody outside of this module see this!
    //     fn get_secret_recipe() -> String {
    //         String::from("Ginger")
    //     }
    //
    //     fn make_sausage() {
    //         get_secret_recipe();
    //         println!("sausage!");
    //     }
    // }
    //
    // sausage_factory::make_sausage();

    mod sausage_factory {
        // Don't let anybody outside of this module see this!
        fn get_secret_recipe() -> String {
            String::from("Ginger")
        }

        pub fn make_sausage() {
            get_secret_recipe();
            println!("sausage!");
        }
    }

    sausage_factory::make_sausage();
}

pub fn bringing_item_into_scope() {
    // Complete this use statement
    // use ???
    //
    // match SystemTime::now().duration_since(UNIX_EPOCH) {
    //     Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
    //     Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    // }

    use std::time::{SystemTime, UNIX_EPOCH};

    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
