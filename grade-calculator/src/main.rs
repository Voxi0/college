use std::{fs::File, io::{self, Write}, path::Path};

// Constants
const MIN_STUDENTS_IN_CLASS: u8 = 6;
const RESULTS_OUTPUT_FILE: &str = "results.txt";

// A student consisting of their name, test score and grade
struct Student {
    name: String,
    score: Score,
    grade: Grade,
}

#[derive(Copy, Clone)]
struct Score(u8);
impl Score {
    fn new(value: u8) -> Result<Self, String> {
        if value <= 100 {
            Ok(Self(value))
        }
        else {
            Err("Score must be between 0 and 100".to_string())
        }
    }

    fn value(&self) -> u8 {self.0}
}

// Grade
#[derive(Default)]
enum Grade {
    #[default]
    Unmarked,
    Pass,
    Merit,
    Distinction,
    Fail,
}
impl Grade {
    fn from_score(score: Score) -> Self {
        match score.value() {
            0..=39 => Grade::Fail,
            40..=50 => Grade::Pass,
            51..=69 => Grade::Merit,
            70..=100 => Grade::Distinction,
            _ => Grade::Unmarked,
        }
    }

    fn to_str(&self) -> &str {
        match self {
            Grade::Unmarked => "Unmarked",
            Grade::Fail => "Fail",
            Grade::Pass => "Pass",
            Grade::Merit => "Merit",
            Grade::Distinction => "Distinction",
        }
    }
}

// Main
fn main() -> io::Result<()> {
    let mut students: Vec<Student> = Vec::new();

    // Ask for the number of students
    let mut num_students: u8 = prompt_num("Enter Number of Students: ");
    num_students = num_students.clamp(MIN_STUDENTS_IN_CLASS, u8::MAX);
    if num_students < 6 {
        println!("Less than minimum number of students, clamping to the minimum of 6 students");
    }

    // Store individual student information in the vector of students
    println!();
    for _ in 0..num_students {
        // Name
        // `.read_line()` also puts the newline/return into the input buffer so we must trim it
        prompt("Student Name: ");
        let mut student_name: String = String::new();
        io::stdin()
            .read_line(&mut student_name)
            .expect("[FATAL ERROR] Failed to read line input");
        student_name = student_name.trim().to_string();

        // Test result which will be clamped to 100 always
        let score: Score = loop { 
            let score: u8 = prompt_num("Enter Test Score: ");
            match Score::new(score) {
                Ok(num) => break num,
                Err(why) => println!("{why}"),
            };
        };

        // Add a new student to the vector of students
        students.push(Student {
            name: student_name,
            score: score,
            grade: Grade::from_score(score),
        });

        println!();
    }

    // Sort student scores from largest to smallest
    students.sort_by(|a, b| b.score.value().cmp(&a.score.value()));

    // Write all student results to a file
    let results_db_path = Path::new(RESULTS_OUTPUT_FILE);
    let mut results_db_file = match File::create(&results_db_path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't create {}: {}", results_db_path.display(), why),
    };
    for (index, student) in students.iter().enumerate() {
        writeln!(results_db_file, "Student Name ({}): {}", index, student.name)?;
        writeln!(results_db_file, "Student Score ({}): {}", index, student.score.value())?;
        writeln!(results_db_file, "Student Grade ({}): {}", index, student.grade.to_str())?;
        writeln!(results_db_file)?;
    }

    // End of program
    return Ok(());
}

/*
    Helper functions
*/

/*
    Prompt the user for some input
    The output buffer has to be flushed so print can show a message
    If this isn't done, no prompt will be shown till user does some input
    So this allows prompting and taking input on the same line visually if that makes sense
*/
fn prompt<T: std::fmt::Display>(msg: T) {
    print!("{msg}");
    io::stdout().flush().unwrap();
}

/*
    Prompts for a number
    We just keep asking for a number till it's valid
*/
fn prompt_num<T: std::str::FromStr>(msg: &str) -> T {
    loop {
        prompt(msg);
        let mut input_buf: String = String::new();
        io::stdin()
            .read_line(&mut input_buf)
            .expect("[ FATAL ] Failed to read line");
        
        match input_buf.trim().parse::<T>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Failed to parse user input, please enter a valid whole number");
                continue;
            },
        }
    }
}
