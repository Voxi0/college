/*
    This program inputs a customer's name, ticket quantity and ticket price before displaying the total cost.

    The problem can be broken down into smaller problems and after answering those smaller problems, the main problem can be solved.
    This also will work as the algorithm since doing each of these step by step in the given order will give us the desired output

    1. We need to input the customer name which is a string
    2. We also need to input ticket quantity and price and then convert it to numerical datatypes since input is a string
    3. Calculate total cost by multiplying the quantity and price of each ticket
    4. Display output, showing the customer's name and how much they need to pay

    The only patterns I see here is the inputs being string before they get converted into proper datatypes

    All information is relevant
*/

#[path = "../../utils.rs"]
mod utils;
use utils::{prompt_str, prompt_num};

fn main() {
    let customer_name: String = prompt_str("Enter Name: ");

    // We made this a float so we can enter the precise price but the question didn't specify which
    // type to use or whatever
    let ticket_price: f32 = prompt_num("Enter Ticket Price: ");

    // Calculate total cost
    let ticket_quantity: u8 = prompt_num("Enter Ticket Quantity: ");
    let total_cost: f32 = ticket_price * ticket_quantity as f32;

    // Final output
    println!("{customer_name}'S RECEIPT");
    println!("Total Cost: {ticket_price} * {ticket_quantity} = {total_cost}");
}
