// Helper functions
#[path = "../../utils.rs"]
mod utils;
use utils::{prompt, prompt_num};

// Only need this for reading a single character without requiring user to press enter
// Wish I could just stick to the standard library for something so simple but sadly no
use console::Term;

// Main
fn main() -> std::io::Result<()> {
    // Initialize terminal
    let term = Term::stdout();

    // Get user age
    let user_age: u8 = prompt_num("Enter Your Age: ");

    // Calculate ticket price
    let ticket_price: u8 = {
        // Calculate ticket price based on age
        let base_price = match user_age {
            // People under 16
            0..=15 => 6,

            // People aged 16 to 64
            16..=64 => 10,

            // People over the age of 64
            _ => 7,
        };

        // Add more money if the user wants popcorn
        let popcorn_price = loop {
            prompt("Would you like some popcorn? (y/n): ");
            match term.read_char()?.to_ascii_lowercase() {
                'y' => break 4,
                'n' => break 0,
                _ => {
                    println!("\nInvalid Input!");
                    continue;
                },
            };
        };

        // Return total price
        base_price + popcorn_price
    };
    

    // Display total price and terminate program
    println!("\nTotal Price = {ticket_price}");
    return Ok(());
}
