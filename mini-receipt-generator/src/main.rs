// Apparently, upper-casing the first letter of a word is an unstable feature
#![feature(titlecase)]

// Helper functions
#[path = "../../utils.rs"]
mod utils;
use utils::prompt_input;

mod receipt;
use receipt::Receipt;

fn input_item() -> (String, f32, i32) {
    // Ask user for the item name, we uppercase the first letter of the name
    let mut item_name: String = String::new();
    prompt_input("Enter Item Name: ", &mut item_name);
    item_name = item_name.word_to_titlecase();

    let item_price: f32 = loop {
        let mut item_price_str: String = String::new();
        prompt_input("Enter Item Price: ", &mut item_price_str);
        if let Ok(price) = item_price_str.trim().parse::<f32>() {
            break price;
        } else {
            println!("Please enter a valid number!");
        }
    };

    let item_quantity: i32 = loop {
        let mut item_quantity_str: String = String::new();
        prompt_input("Enter Item Quantity: ", &mut item_quantity_str);
        if let Ok(price) = item_quantity_str.trim().parse::<i32>() {
            break price;
        } else {
            println!("Please enter a valid number!");
        }
    };

    return (item_name, item_price, item_quantity);
}

fn main() {
    let mut receipt = Receipt::default();

    let mut running = true;
    while running {
        let (item_name, item_price, item_quantity) = input_item();
        receipt.add_item(item_name.as_str(), item_price, item_quantity);

        // Ask user if they want to keep adding items
        let mut running_str: String = String::new();
        prompt_input("Add another item? (yes/no): ", &mut running_str);
        println!("");
        running_str = running_str.to_lowercase();
        match running_str.as_str() {
            "yes" | "ye" | "y" => continue,
            "no" | "n" => running = false,
            _ => {},
        }
    }

    println!("");
    receipt.print();
}
