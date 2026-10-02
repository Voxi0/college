use std::io::{self, Write};
use std::any::type_name;

// Helper functions
// Only display a prompt leaving the programmer to decide how to handle input
pub fn prompt<T: std::fmt::Display>(msg: T) {
    // Display the prompt
    // Not flushing the buffer stops the message from displaying until the user inputs something
    print!("{msg}");
    io::stdout().flush().expect("[FATAL] Failed to flush stdout");
}

// Handle inputting a string
pub fn prompt_str<T: std::fmt::Display>(msg: T, input_buf: &mut String) {
    prompt(msg);

    // Read a line of input until user enters newline
    io::stdin()
        .read_line(input_buf)
        .expect("[FATAL] Read-line failed");

    // Get rid of trailing whitespaces and the final newline
    // Yes, Rust puts the final newline entered by the user into the buffer
    *input_buf = input_buf.trim().to_string();
}

// Handle inputting a number of any kind
pub fn prompt_num<T: std::str::FromStr>(msg: &str) -> T {
    loop {
        let mut num_str: String = String::new();
        prompt_str(msg, &mut num_str);
        match num_str.trim().parse::<T>() {
            Ok(num) => break num,
            Err(_) => {
                let full_type = type_name::<T>();
                match full_type {
                    "u8" | "core::primitive::u8" => {
                        println!("Invalid number! Please enter a whole number {} to {}\n", u8::MIN, u8::MAX);
                    },
                    "i8" | "core::primitive::i8" => {
                        println!("Invalid number! Please enter a whole number {} to {}\n", i8::MIN, i8::MAX);
                    },

                    "u16" | "core::primitive::u16" => {
                        println!("Invalid number! Please enter a whole number {} to {}\n", u16::MIN, u16::MAX);
                    },
                    "i16" | "core::primitive::i16" => {
                        println!("Invalid number! Please enter a whole number {} to {}\n", i16::MIN, i16::MAX);
                    },

                    "u32" | "core::primitive::u32" => {
                        println!("Invalid number! Please enter a whole number {} to {}\n", u32::MIN, u32::MAX);
                    },
                    "i32" | "core::primitive::i32" => {
                        println!("Invalid number! Please enter a whole number {} to {}\n", i32::MIN, i32::MAX);
                    },

                    "f32" | "core::primitive::f32" => {
                        println!("Invalid number! Please enter a decimal number {} to {}\n", f32::MIN, f32::MAX);
                    },
                    "f64" | "core::primitive::f64" => {
                        println!("Invalid number! Please enter a decimal number {} to {}\n", f64::MIN, f64::MAX);
                    },

                    _ => {
                        let short_name = full_type.rsplit("::").next().unwrap_or("number");
                        println!("Invalid number! Please enter a valid {short_name}\n");
                    },
                }
            },
        }
    }
}
