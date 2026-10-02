// Helper functions
#[path = "../../utils.rs"]
mod utils;
use console::Term;
use utils::{prompt, prompt_num};

// Convert celsius/fahrenheit to the other unit and return it
fn to_fahrenheit(temp_celsius: f32) -> f32 {
    (temp_celsius * 1.8) + 32.0
}
fn to_celsius(temp_fahrenheit: f32) -> f32 {
    (temp_fahrenheit - 32.0) / 1.8
}

fn main() -> std::io::Result<()> {
    // Initialize terminal
    let term: Term = Term::stdout();

    // Main loop
    let mut running: bool = true;
    while running {
        // Ask user to specify what temperature unit they want to convert from
        prompt("Choose Temperature Unit (c/f): ");
        match term.read_char()?.to_ascii_lowercase() {
            // If user wants to input celsius, convert to fahrenheit
            'c' => {
                println!();
                let celsius: f32 = prompt_num("Enter Temperature in Celsius: ");
                println!("{celsius}°F is {}", to_fahrenheit(celsius));
            },

            // If user wants to input fahrenheit, convert to celsius
            'f' => {
                println!();
                let fahrenheit: f32 = prompt_num("Enter Temperature in Fahrenheit: ");
                println!("{fahrenheit}°F is {}", to_celsius(fahrenheit));
            },

            // Invalid input, just move to next line and prompt them again till they either press `c` or `f`
            _ => {
                println!();
                continue
            },
        }

        // Ask user if they want to convert again
        prompt("Convert Another Temperature? (y/n): ");
        loop {
            match term.read_char()?.to_ascii_lowercase() {
                // Print TWO newlines before next iteration
                'y' => {
                    println!("\n");
                    break;
                },

                // Exit program
                'n' => {
                    running = false;
                    break;
                },

                _ => {},
            }
        }
    }

    return Ok(());
}
