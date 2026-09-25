#![allow(non_snake_case)]
use std::io::{self, Write};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct StudentRecord {
    username: String,
    password: String,
}

// Display a simple prompt
fn prompt(string: &str) {
    print!("{string}");

    // Not doing this causes the `print!` to show it's message only after user input for some reason
    io::stdout().flush().unwrap();
}

// Reads a line of input, trims the newline from the end and returns it
fn readLine() -> String {
    let mut string: String = String::new();
    io::stdin()
        .read_line(&mut string)
        .expect("[USER INPUT ERROR]");
    string = string.trim().to_string();
    return string;
}

fn main() -> io::Result<()>{
    // Check if student exists in the college database
    let mut database = csv::Reader::from_path("db.csv")?;
    let databaseStartPos = database.position().clone();

    // Main loop
    let mut verified: bool = false;
    while !verified {
        // Get student username and password
        prompt("Enter Your Username: ");
        let username: String = readLine();
        prompt("Enter Your Password: ");
        let password: String = readLine();

        // Read the college's student database to check username and password
        for result in database.deserialize() {
            let record: StudentRecord = result?;
            if record.username == username {
                if record.password == password {
                    verified = true;
                    println!("Login Successful\n");
                    break;
                } else {
                    println!("Incorrect Password\n");
                    break;
                }
            }
        }

        // Read the database from the beginning again
        database.seek(databaseStartPos.clone())?;
    }

    return Ok(());
}
