use std::{error::Error, fs};

pub fn print_output(problem_name: String, sum: isize) -> Result<(), Box<dyn Error>> {
    println!("{:}: {:}", problem_name, sum);

    let output_file = "outputs/".to_string() + &problem_name + ".txt";

    let result = fs::write(output_file, sum.to_string());

    match result {
        Ok(_) => return Ok(()),
        Err(error) => return Err(Box::new(error)),
    }
}
