use crate::utils::prompt;

#[derive(Default)]
pub struct User {
    name: String,
    age: u8,
    hometown: String,
    fav_tech: String,
    intended_career: String,
}

impl User {
    // Creates a new user to return either the user or an error
    pub fn new() -> Result<Self, String> {
        let mut user: Self = Self::default();

        // Ask for user's name
        prompt("Enter Your Name: ", &mut user.name)?;

        // User age - We have to parse the string to a u8
        let mut user_age: String = String::new();
        user.age = loop {
            prompt("Enter Your Age: ", &mut user_age)?;
            match user_age.trim().parse::<u8>() {
                Ok(age) => break age,
                Err(_) => {
                    println!("Invalid age! Please enter a number 0-255");
                    user_age.clear();
                    continue;
                },
            }
        };

        // Ask user for their other details
        prompt("Enter Your Hometown: ", &mut user.hometown)?;
        prompt("Enter Your Favourite Tech: ", &mut user.fav_tech)?;
        prompt("What's Your Intended Career? ", &mut user.intended_career)?;

        return Ok(user);
    }

    // Display user information nicely
    pub fn show(&self) {
        println!("User: {}", self.name);
        println!("Age: {}", self.age);
        println!("Hometown: {}", self.hometown);
        println!("Favourite Tech: {}", self.fav_tech);
        println!("Intended Career: {}", self.intended_career);
    }
}
