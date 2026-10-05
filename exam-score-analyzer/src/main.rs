// Helper functions
#[path = "../../utils.rs"]
mod utils;
use utils::{prompt_str, prompt_num};

// Constants
const NUM_SCORES: usize = 3;

fn main() {
    // Input student name
    let student_name: String = prompt_str("Enter Your Name: ");

    // Input student's exam scores
    let mut exam_scores: [u8; NUM_SCORES] = [0; NUM_SCORES];
    for (index, item) in exam_scores.iter_mut().enumerate() {
        *item = prompt_num(format!("Enter Score {index}: ").as_str());
    }

    // Calculate total and average score
    let mut total_score: u8 = 0;
    for score in exam_scores {total_score += score}
    let average_score: f32 = total_score as f32 / NUM_SCORES as f32;

    // Output
    println!("");
    println!("Student: {student_name}");
    println!("Total Score: {total_score}");
    println!("Average Score: {:.2}", average_score);
}
