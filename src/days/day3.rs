use crate::helpers::output::print_output;
use regex::Regex;
use std::{error::Error, fs::read_to_string};

fn read_input() -> Result<String, Box<dyn Error>> {
    let input_file = "inputs/day3a.txt";
    let text = read_to_string(input_file)?;
    return Ok(text);
}

fn add_mults_in_text(text: &String, regex: &Regex) -> Result<(isize), Box<dyn Error>> {
    let mut sum = 0;

    let captured: regex::CaptureMatches<'_, '_> = regex.captures_iter(text);
    for m in captured {
        let first: isize = m.name("first").unwrap().as_str().parse()?;
        let second: isize = m.name("second").unwrap().as_str().parse()?;

        sum += first * second;
    }

    return Ok(sum);
}

#[test]
fn test_regex() {
    let re: Regex = Regex::new(r"mul\((?<first>[0-9]+),(?<second>[0-9]+)\)").unwrap();
    let hay = "^<when()]'>mul(721,815)who()".to_string();
    let sum = add_mults_in_text(&hay, &re).unwrap();

    println!("{:?}", sum);
}

#[test]
pub fn day3a() -> Result<(), Box<dyn Error>> {
    let text = read_input()?;
    let re = Regex::new(r"mul\((?<first>[0-9]+),(?<second>[0-9]+)\)")?;

    let sum: isize = add_mults_in_text(&text, &re)?;

    return print_output("day3a".to_string(), sum);
}

#[test]
pub fn day3b() -> Result<(), Box<dyn Error>> {
    let text = read_input()?;
    let re = Regex::new(r"mul\((?<first>[0-9]+),(?<second>[0-9]+)\)")?;

    let mut sum: isize = 0;
    for dos in text.split("do()") {
        let donts: Vec<&str> = dos.split("don't()").collect();
        sum += add_mults_in_text(&donts[0].to_string(), &re)?;
    }

    return print_output("day3a".to_string(), sum);
}
