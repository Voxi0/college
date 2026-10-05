/*
    Looking at the specifications, I know what my program has to do.

    1. Record the learner's name, course name, course fee and number of instalments
    2. Calculate the amount to pay per instalment
    3. Display the calculated amount

    So we just input two strings for learner's name and their course name
    while course fee and number of instalments will be unsigned integers since they
    should never be negative.

    The calculated output will actually be a float so we can display a precise
    number to the user since not all divisions will be perfect. And yes it's
    calculated by dividing the total course fee by the number of instalments
    that the learner will be paying.

    After making any program, it's good to test with both valid and invalid data
    to ensure the code is handling common edge cases and such properly by which I
    mean it handles errors properly instead of crashing at everything.
*/

#[path = "../../utils.rs"]
mod utils;
use utils::{prompt_str, prompt_num};

fn main() {
    // Learner and course name
    let learner_name: String = prompt_str("Enter Your Name: ");
    let course_name: String = prompt_str("Enter Course Name: ");

    // `u16` represents upto about 32,767 which should be plenty enough for these
    let course_fee: u16 = prompt_num("Enter Course Fee: ");
    let num_instalments: u16 = prompt_num("Enter Number of Instalments: ");

    // Dividing the total course fee by the number of instalments will give us how much to pay per instalment
    // We use `f32` here so we can get a precise amount to pay
    let amount_paid_per_instalment: f32 = course_fee as f32 / num_instalments as f32;

    // Display output to the user
    println!();
    println!("{learner_name} - {course_name}");
    println!("Total Course Fee: {course_fee}");
    println!("Number of Instalments: {num_instalments}");
    println!("Amount to Pay Per Instalment: {amount_paid_per_instalment}");
}
