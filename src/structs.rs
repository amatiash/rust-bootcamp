pub fn code() {
    // struct Product {
    //     name: String,
    //     price: f32,
    //     in_stock: bool
    // }
    //
    // impl Product {
    //     fn new(name: String, price: f32) -> Product {
    //         Product {
    //             name: name,
    //             price: price,
    //             in_stock: true
    //         }
    //     }
    //     fn get_default_sales_tax() -> f32 {
    //         0.1
    //     }
    //     fn calculate_sales_tax(&self) -> f32 {
    //         self.price * Product::get_default_sales_tax()
    //     }
    //     fn set_price(&mut self, price: f32) {
    //         self.price = price;
    //     }
    //     fn buy(self) -> i32 {
    //         let name = self.name;
    //         println!("{name} was bought!");
    //         123
    //     }
    // }
    //
    // let mut book = Product::new(
    //     String::from("Book"),
    //     30.0
    // );
    // let sales_tax = book.calculate_sales_tax();
    // println!("sales tax: {}", sales_tax);
    // book.set_price(1.0);
    // book.buy();
}

pub fn struct_definition() {
    // struct User {
    //     // Something goes here
    // }
    // let user = User {
    //     name: String::from("Tom Riddle"),
    //     age: 17u8,
    // };
    // println!("User's name: {}", user.name);
    // println!("User's age: {}", user.age);
    
    struct User {
        name: String,
        age: u8,
    }
    let user = User {
        name: String::from("Tom Riddle"),
        age: 17u8,
    };
    println!("User's name: {}", user.name);
    println!("User's age: {}", user.age);
}

pub fn mutating_structs() {
    // Make the following code compile.
    // struct ShopItem {
    //     name: String,
    //     quantity: u32,
    //     in_stock: bool,
    // }
    // 
    // let item = ShopItem {
    //     name: String::from("Socks"),
    //     quantity: 200,
    //     in_stock: true,
    // };
    // // 50 pairs of socks were sold
    // item.quantity -= 50;
    // if item.quantity == 0 {
    //     item.in_stock = false;
    // }
    // println!("{} is in stock: {}", item.name, item.in_stock);

    struct ShopItem {
        name: String,
        quantity: u32,
        in_stock: bool,
    }

    let mut item = ShopItem {
        name: String::from("Socks"),
        quantity: 200,
        in_stock: true,
    };
    // 50 pairs of socks were sold
    item.quantity -= 50;
    if item.quantity == 0 {
        item.in_stock = false;
    }
    println!("{} is in stock: {}", item.name, item.in_stock);
}

pub fn structs_and_functions() {
    // Complete the function signatures and make the code compile.
    // struct ShopItem {
    //     name: String,
    //     quantity: u32,
    // }
    // fn create_item(name: &str, quantity: u32) -> {
    //     ShopItem {
    //         name: name.to_string(),
    //         quantity, // notice how struct initializations can be shortened when variable and field have same name
    //     }
    // }
    // fn is_in_stock(item) -> bool {
    //     item.quantity > 0
    // }
    // let item = create_item("Socks", 200);
    // let in_stock = is_in_stock(&item);
    // println!("{} is in stock: {in_stock}", item.name);

    struct ShopItem {
        name: String,
        quantity: u32,
    }
    fn create_item(name: &str, quantity: u32) -> ShopItem {
        ShopItem {
            name: name.to_string(),
            quantity, // notice how struct initializations can be shortened when variable and field have same name
        }
    }
    fn is_in_stock(item: &ShopItem) -> bool {
        item.quantity > 0
    }
    let item = create_item("Socks", 200);
    let in_stock = is_in_stock(&item);
    println!("{} is in stock: {in_stock}", item.name);
}
