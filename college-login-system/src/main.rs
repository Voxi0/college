#[path = "../../utils.rs"]
mod utils;
use utils::prompt_str;

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

fn main() -> eyre::Result<()>{
    // Install `color-eyre` panic/errors handlers
    color_eyre::install()?;

    // Check if student exists in the college database
    let mut database = csv::Reader::from_path("db.csv")
        .wrap_err("Failed to open `db.csv` as it doesn't exist")?;
    database.headers().wrap_err("Failed to read headers from `db.csv`")?; // Skip the headers before recording the start position
    let db_start_pos = database.position().clone();

    // Main loop
    let mut verified: bool = false;
    let mut num_tries: u8 = 0;
    while !verified && num_tries < 3 {
        num_tries += 1;

        // Get student username and password
        let username: String = prompt_str("Enter Your Username: ");
        let password: String = prompt_str("Enter Your Password: ");

        // Read the college's student database to check username and password
        let mut user_found: bool = false;
        for result in database.deserialize() {
            let record: StudentRecord = result.wrap_err("Failed to parse row from `db.csv`")?;

            // Check if user exists before checking the password
            user_found = record.username == username;
            if !user_found {continue}
            verified = (record.password == password) && user_found;

            // User is found so we exit the loop early
            break;
        }

        // User not found or incorrect password
        if !user_found {
            println!("Invalid Username, {num_tries} Out of 3 Tries Remaining");
        } else if !verified {
            println!("Invalid Password, {num_tries} Out of 3 Tries Remaining");
        }

        // Read the database from the beginning again
        database.seek(db_start_pos.clone())?;

        // Newline before retrying just because
        println!("");
    }

    // Check if user is verified finally
    if !verified {
        println!("Login Failed. Account Has Been Locked");
    } else {
        println!("Login Successful");
    }

    // End of program
    return Ok(());
}
