// Broken Python code
// Not only do I have to fix the code, I'll also have to implement it in Rust

// Line 5 and 7 are missing commas
// Line 3 is attempting to add an integer to a string since `input` returns a string. Just convert
// the age input to `int` right away
/*
   1. student_name = input("Enter name: ")
   2. age = input("Enter age: ")
   3. next_age = age + 1
   4. score = float(input("Enter score: "))
   5. print("Student:" student_name)
   6. print("Next age:", next_age)
   7. print("Score:" score)
*/

#[path = "../../utils.rs"]
mod utils;
use utils::{prompt_str, prompt_num};

fn main() {
    // Input
    let mut student_name: String = String::new();
    prompt_str("Enter name: ", &mut student_name);
    let age: u8 = prompt_num("Enter age: ");
    let next_age: u8 = age + 1;
    let score: f32 = prompt_num("Enter score: ");

    // Output
    println!("Student: {student_name}");
    println!("Next age: {next_age}");
    println!("Score: {score}");
}
