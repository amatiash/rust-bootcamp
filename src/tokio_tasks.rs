#![allow(dead_code, unused_variables)]

pub fn code() {
    // use std::time::Duration;
    // use tokio::time::sleep;
    //
    // #[tokio::main(flavor = "current_thread")]
    // async fn main() {
    //     let mut handles = vec![];
    //
    //     for i in 0..2 {
    //         let handle = tokio::spawn(async move {
    //             my_function(i).await;
    //         });
    //         handles.push(handle);
    //     }
    //
    //     for handle in handles {
    //         handle.await.unwrap();
    //     }
    // }
    //
    // async fn my_function(i: i32) {
    //     println!("[{i}] I'm an async function!");
    //     let s1 = read_from_database().await;
    //     println!("[{i}] First result: {s1}");
    //     let s2 = read_from_database().await;
    //     println!("[{i}] Second result: {s2}");
    // }
    //
    // async fn read_from_database() -> String {
    //     sleep(Duration::from_millis(50)).await;
    //     "DB Result".to_owned()
    // }
}

pub async fn awaiting_tasks() {
    // Fix the code to make it compile.
    // use std::time::Duration;
    // use tokio::time::sleep;
    //
    // struct Employee {
    //     id: u32,
    //     name: String,
    //     salary: f32,
    // }
    //
    // impl Employee {
    //     fn new(id: u32, name: &str, salary: f32) -> Self {
    //         Self {
    //             id,
    //             name: name.to_string(),
    //             salary,
    //         }
    //     }
    // }
    //
    // async fn print_details(id: u32) -> Result<(), String> {
    //     let emp = read_details_from_db(id).await?;
    //     println!("Id: {}, Name: {}, Salary: {}", emp.id, emp.name, emp.salary);
    //     Ok(())
    // }
    //
    // async fn read_details_from_db(id: u32) -> Result<Employee, String> {
    //     // dummy read from database
    //     sleep(Duration::from_millis(1000)).await;
    //     let database = [
    //         Employee::new(1, "Alice", 98000.0),
    //         Employee::new(2, "Bob", 95000.0),
    //         Employee::new(3, "Cindy", 95000.0),
    //         Employee::new(4, "Daniel", 88000.0),
    //     ];
    //     for emp in database {
    //         if id == emp.id {
    //             return Ok(emp);
    //         }
    //     }
    //     Err(format!("Employee record for id {} not present", id))
    // }
    //
    // let ids = [1, 2, 4, 5, 9, 10];
    // let mut handles = Vec::new();
    // for id in ids {
    //     let handle = tokio::spawn(async move {
    //         let res = print_details(id).await;
    //         if let Err(e) = res {
    //             println!("{e}");
    //         }
    //     });
    //     handles.push(handle);
    // }
    // for handle in handles {
    //     handle.join().unwrap();
    // }

    use std::time::Duration;
    use tokio::time::sleep;

    struct Employee {
        id: u32,
        name: String,
        salary: f32,
    }

    impl Employee {
        fn new(id: u32, name: &str, salary: f32) -> Self {
            Self {
                id,
                name: name.to_string(),
                salary,
            }
        }
    }

    async fn print_details(id: u32) -> Result<(), String> {
        let emp = read_details_from_db(id).await?;
        println!("Id: {}, Name: {}, Salary: {}", emp.id, emp.name, emp.salary);
        Ok(())
    }

    async fn read_details_from_db(id: u32) -> Result<Employee, String> {
        // dummy read from database
        sleep(Duration::from_millis(1000)).await;
        let database = [
            Employee::new(1, "Alice", 98000.0),
            Employee::new(2, "Bob", 95000.0),
            Employee::new(3, "Cindy", 95000.0),
            Employee::new(4, "Daniel", 88000.0),
        ];
        for emp in database {
            if id == emp.id {
                return Ok(emp);
            }
        }
        Err(format!("Employee record for id {} not present", id))
    }

    let ids = [1, 2, 4, 5, 9, 10];
    let mut handles = Vec::new();
    for id in ids {
        let handle = tokio::spawn(async move {
            let res = print_details(id).await;
            if let Err(e) = res {
                println!("{e}");
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
