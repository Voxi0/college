use std::io::{self, Write};

pub fn prompt(msg: &str, input_buf: &mut String) -> Result<(), String> {
    // We flush `stdout` because `print!` doesn't show the message until after input
    // I really don't know why but whatever
    print!("{msg}");
    io::stdout().flush().expect("Failed to flush stdout");

    // Read line
    io::stdin()
        .read_line(input_buf)
        .expect("Failed to read line");

    // Remove the newline
    // `read_line` also puts the newline in the input buffer
    *input_buf = input_buf.trim().to_string();

    return Ok(());
}
