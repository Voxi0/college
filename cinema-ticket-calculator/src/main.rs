#![allow(non_snake_case)]
use std::{io, io::Write};
use console::Term;

// Small helper function
fn prompt(msg: &str) {
    print!("{}", msg);

    // Requires `std::io::Write` to be in scope
    io::stdout().flush().unwrap();
}

// Main
fn main() -> std::io::Result<()> {
    // Initialize terminal
    let term = Term::stdout();

    // Get user age
    let userAge: u8 = loop {
        let mut userAgeStr: String = String::new();
        prompt("Enter Your Age: ");
        io::stdin()
            .read_line(&mut userAgeStr)
            .expect("[USER INPUT READLINE FAILURE]");
        match userAgeStr.trim().parse() {
            Ok(num) => break num,
            Err(_) => {
                println!("Invalid Number! Please Try Again");
                continue;
            },
        };
    };

    // Calculate ticket price
    let ticketPrice: u8 = {
        // Calculate ticket price based on age
        let basePrice = match userAge {
            // People under 16
            0..=15 => 6,

            // People aged 16 to 64
            16..=64 => 10,

            // People over the age of 64
            _ => 7,
        };

        // Add more money if the user wants popcorn
        let popcornPrice = loop {
            prompt("Would you like some popcorn? (y/n): ");
            match term.read_char()? {
                'y' | 'Y' => break 4,
                'n' | 'N' => break 0,
                _ => {
                    println!("\nInvalid Input!");
                    continue;
                },
            };
        };

        // Return total price
        basePrice + popcornPrice
    };
    

    // Display total price and terminate program
    println!("\nTotal Price = {ticketPrice}");
    return Ok(());
}
