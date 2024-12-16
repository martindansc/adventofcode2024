use crate::helpers::{cmath::n_digits, output::print_output};
use std::{error::Error, fs::read_to_string};

fn read_input() -> Result<Vec<isize>, Box<dyn Error>> {
    let input_file = "inputs/day11a.txt";

    for line in read_to_string(input_file)?.lines() {
        let row: Vec<isize> = line
            .split_terminator(" ")
            .map(|x| x.parse::<isize>().expect("Input should be correct"))
            .collect();

        return Ok(row);
    }

    return Err(Box::from("Should have one row"));
}

pub fn transform(val: isize) -> Vec<isize> {
    let mut values: Vec<isize> = Vec::new();

    let digits = n_digits(val);

    if val == 0 {
        values.push(1);
    } else if digits % 2 == 0 {
        values.push(val / 10_isize.pow((digits / 2) as u32));
        values.push(val % 10_isize.pow((digits / 2) as u32));
    } else {
        values.push(val * 2024);
    }

    return values;
}

pub fn iterate(old_list: &mut Vec<isize>) -> Vec<isize> {
    let mut new_list: Vec<isize> = Vec::new();

    while let Some(val) = old_list.pop() {
        new_list.extend(transform(val));
    }

    return new_list;
}

#[test]
pub fn day11a() -> Result<(), Box<dyn Error>> {
    let mut list = read_input()?;

    println!("{:?}", list);

    for _i in 0..75 {
        list = iterate(&mut list);
    }


    return print_output("day11a".to_string(), list.len() as isize);
}

#[test]
pub fn test() {
    assert_eq!(transform(1), vec![2024]);
    assert_eq!(transform(17), vec![1, 7]);
    assert_eq!(transform(0), vec![1]);
    assert_eq!(transform(125), vec![125 * 2024]);
}
