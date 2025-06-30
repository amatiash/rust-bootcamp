use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

fn main() {
    // Create database and navigator
    let db = Rc::new(JiraDatabase::new("data/db.json".to_string()));
    let mut navigator = Navigator::new(db);

    loop {
        clearscreen::clear().unwrap();

        // Get current page from navigator. If there is no current page exit the loop.
        let current_page = match navigator.get_current_page() {
            Some(page) => page,
            None => break,
        };

        // Render page
        if let Err(error) = current_page.draw_page() {
            println!(
                "Error rendering page: {}\nPress any key to continue...",
                error
            );
            wait_for_key_press();
            continue;
        }

        // Input loop - keep asking for input until we get a valid action
        loop {
            // Get user input
            let input = get_user_input();

            // Pass input to page's input handler
            match current_page.handle_input(input.trim()) {
                Ok(Some(action)) => {
                    if let Err(error) = navigator.handle_action(action) {
                        println!(
                            "Error processing action: {}\nPress any key to continue...",
                            error
                        );
                        wait_for_key_press();
                    }
                    break;
                }
                Ok(None) => {
                    continue;
                }
                Err(error) => {
                    println!(
                        "Error handling input: {}\nPress any key to continue...",
                        error
                    );
                    wait_for_key_press();
                    continue;
                }
            }
        }
    }
}
