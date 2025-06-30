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

        // Get user input
        let input = get_user_input();

        // Pass input to page's input handler
        let action = match current_page.handle_input(&input) {
            Ok(Some(action)) => action,
            Ok(None) => continue, // No action to process, continue to next iteration
            Err(error) => {
                println!(
                    "Error handling input: {}\nPress any key to continue...",
                    error
                );
                wait_for_key_press();
                continue;
            }
        };

        // Let the navigator process the action
        if let Err(error) = navigator.handle_action(action) {
            println!(
                "Error processing action: {}\nPress any key to continue...",
                error
            );
            wait_for_key_press();
        }
    }
}
