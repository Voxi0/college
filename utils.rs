use std::io::{self, Write};

// Helper functions
pub fn prompt(msg: &str) {
    // Display the prompt
    // Not flushing the buffer stops the message from displaying until the user inputs something
    print!("{msg}");
    io::stdout().flush().expect("[FATAL] Failed to flush stdout");
}
pub fn prompt_input(msg: &str, input_buf: &mut String) {
    prompt(msg);

    // Read a line of input until user enters newline
    io::stdin()
        .read_line(input_buf)
        .expect("[FATAL] Read-line failed");

    // Get rid of trailing whitespaces and the final newline
    // Yes, Rust puts the final newline entered by the user into the buffer
    *input_buf = input_buf.trim().to_string();
}
