#![allow(non_snake_case)]
use std::io::{self, Write};
use color_eyre::eyre::{self, WrapErr};
use serde::Deserialize;

// Student record type
// Specifies how a single record is laid out in the database
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct StudentRecord {
    username: String,
    password: String,
}

// Display a simple prompt
fn prompt(string: &str) -> eyre::Result<()> {
    print!("{string}");

    // Not doing this causes the `print!` to show it's message only after user input for some reason
    io::stdout().flush()?;
    return Ok(());
}

// Reads a line of input, trims the newline from the end and returns it
fn readLine() -> eyre::Result<String> {
    let mut string: String = String::new();
    io::stdin().read_line(&mut string)?;
    return Ok(string.trim().to_string());
}

fn main() -> eyre::Result<()>{
    // Install `color-eyre` panic/errors handlers
    color_eyre::install()?;

    // Check if student exists in the college database
    let mut database = csv::Reader::from_path("db.csv")
        .wrap_err("Failed to open `db.csv` as it doesn't exist")?;
    database.headers().wrap_err("Failed to read headers from `db.csv`")?; // Skip the headers before recording the start position
    let databaseStartPos = database.position().clone();

    // Main loop
    let mut verified: bool = false;
    let mut numOfTries: u8 = 0;
    while !verified && numOfTries < 3 {
        numOfTries += 1;

        // Get student username and password
        println!("");
        prompt("Enter Your Username: ")?;
        let username: String = readLine()?;
        prompt("Enter Your Password: ")?;
        let password: String = readLine()?;

        // Read the college's student database to check username and password
        let mut userFound: bool = false;
        for result in database.deserialize() {
            let record: StudentRecord = result.wrap_err("Failed to parse row from `db.csv`")?;

            // Check if user exists before checking the password
            userFound = (record.username == username);
            if !userFound continue;
            verified = (record.password == password) && userFound;

            // User is found so we exit the loop early
            break;
        }

        // User not found or incorrect password
        if !userFound {
            println!("Invalid Username");
        } else if !verified {
            println!("Invalid Password");
        }

        // Read the database from the beginning again
        database.seek(databaseStartPos.clone())?;
    }

    // Check if user is verified finally
    if !verified {
        println!("\nLogin Failed. Account Has Been Locked");
    } else {
        println!("\nLogin Successful");
    }

    // End of program
    return Ok(());
}
