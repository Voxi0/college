#[path = "../../utils.rs"]
mod utils;
use utils::prompt_num;

fn main() {
    let exam_score: f32 = prompt_num("Enter Exam Score: ");
    if exam_score >= 50.0 {
        println!("PASS");
    } else {
        println!("NOT YET PASSED");
    }
}
