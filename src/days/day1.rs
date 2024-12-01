use crate::helpers::output::print_output;
use std::{collections::HashSet, error::Error, fs::read_to_string};

fn read_input() -> Result<(Vec<isize>, Vec<isize>), Box<dyn Error>> {
    let input_file = "inputs/day1a.txt";

    let mut list_1: Vec<isize> = Vec::new();
    let mut list_2: Vec<isize> = Vec::new();

    for line in read_to_string(input_file)?.lines() {
        let readed_line = line.to_string();
        let readed_line_split: Vec<&str> = readed_line.split_terminator("   ").collect();

        let val_1: isize = readed_line_split[0].parse()?;
        let val_2: isize = readed_line_split[1].parse()?;

        list_1.push(val_1);
        list_2.push(val_2);
    }

    return Ok((list_1, list_2));
}

#[test]
pub fn day1a() -> Result<(), Box<dyn Error>> {
    let (mut list_1, mut list_2) = read_input()?;

    list_1.sort();
    list_2.sort();

    let mut sum = 0;
    for i in 0..list_1.len() {
        sum += (list_1[i] - list_2[i]).abs();
    }

    return print_output("day1a".to_string(), sum);
}

#[test]
pub fn day1b() -> Result<(), Box<dyn Error>> {
    let (list_1, list_2) = read_input()?;

    let set_list_1: HashSet<isize> = HashSet::from_iter(list_1);

    let mut sum = 0;
    for i in 0..list_2.len() {
        if set_list_1.contains(&list_2[i]) {
            sum += list_2[i];
        }
    }

    return print_output("day1b".to_string(), sum);
}
