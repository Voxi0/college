#[path = "../../utils.rs"]
mod utils;
use utils::{prompt_str, prompt_num};

use chrono::{DateTime, Datelike, Utc};

#[derive(Default)]
pub struct User {
    name: String,
    age: i8,
    hometown: String,
    fav_tech: String,
    intended_career: String,
}

impl User {
    // Creates a new user to return either the user or an error
    pub fn new() -> Result<Self, String> {
        let mut user: Self = Self::default();

        // Ask for user's name
        prompt_str("Enter Your Name: ", &mut user.name);

        // User age - We have to parse the string to a u8
        user.age = prompt_num("Enter Your Age: ");

        // Ask user for their other details
        prompt_str("Enter Your Hometown: ", &mut user.hometown);
        prompt_str("Enter Your Favourite Tech: ", &mut user.fav_tech);
        prompt_str("What's Your Intended Career? ", &mut user.intended_career);

        return Ok(user);
    }

    // Display user information nicely
    pub fn show(&self) {
        println!("[{}'S PROFILE]", self.name.to_uppercase());
        println!("User: {}", self.name);
        println!("Age: {}", self.age);
        println!("Hometown: {}", self.hometown);
        println!("Favourite Tech: {}", self.fav_tech);
        println!("Intended Career: {}", self.intended_career);

        // Age in 5 years, 10 years, and number of years before the user is 30
        let now: DateTime<Utc> = Utc::now();
        let years_before_30: i32 = 30 - self.age as i32;
        println!("Age in 5 Years: {}", self.age + 5);
        println!("Age in 10 years: {}", self.age + 10);
        println!("Will Turn 30 by The Year: {}", now.year() + years_before_30);
    }
}
