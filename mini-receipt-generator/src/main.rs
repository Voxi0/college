// Apparently, upper-casing the first letter of a word is an unstable Rust feature
#![feature(titlecase)]

// Helper functions
#[path = "../../utils.rs"]
mod utils;
use utils::{prompt_str, prompt_num};

mod receipt;
use receipt::Receipt;

fn main() {
    let mut receipt = Receipt::default();
    loop {
        // Ask user for item name
        // We upper-case the first letter of the name (Unstable Rust feature)
        // Stop input if no name entered
        let mut item_name: String = String::new();
        prompt_str("Enter Item Name (Or Leave Blank to Stop): ", &mut item_name);
        if item_name.is_empty() {break}
        item_name = item_name.word_to_titlecase();

        // Self-explanatory
        let item_price: f32 = prompt_num("Enter Item Price: ");
        let item_quantity: i32 = prompt_num("Enter Item Quantity: ");

        // Add item to receipt and print a newline just because it looks nicer
        receipt.add_item(item_name.as_str(), item_price, item_quantity);
        println!("");
    }

    // Newline before displaying receipt
    println!("");
    receipt.print();
}
