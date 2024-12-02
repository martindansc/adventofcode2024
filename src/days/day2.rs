use crate::helpers::output::print_output;
use std::{error::Error, fs::read_to_string};

fn read_input() -> Result<Vec<Vec<isize>>, Box<dyn Error>> {
    let input_file = "inputs/day2a.txt";

    let mut matrix: Vec<Vec<isize>> = Vec::new();

    for line in read_to_string(input_file)?.lines() {
        let readed_line = line.to_string();
        let row: Vec<isize> = readed_line
            .split_terminator(" ")
            .map(|x| x.parse::<isize>().expect("Input should be correct"))
            .collect();

        matrix.push(row);
    }

    return Ok(matrix);
}

pub fn check_pair(previous: &isize, current: &isize, sign: isize) -> bool {
    let value = (previous - current) * sign;
    return value < 4 && value > 0;
}

pub fn check_row_sign(row: &Vec<isize>, tolerance: isize, sign: isize) -> bool {
    if row.len() < (tolerance + 2).try_into().unwrap() {
        return true;
    }

    let mut previous = row.get(0);
    let mut current = row.get(1);

    let mut i = 2;
    let mut n_errors = 0;

    while current.is_some() {
        if !check_pair(previous.unwrap(), current.unwrap(), sign) {
            n_errors += 1
        } else {
            previous = current;
        }

        current = row.get(i);
        i += 1;

        if n_errors > tolerance {
            return false;
        }
    }

    return true;
}

// Inneficient function, it only works with tolerances 0 or 1
pub fn check_row(row: &Vec<isize>, tolerance: isize) -> bool {
    let mut reverse_row = row.clone();
    reverse_row.reverse();

    return check_row_sign(row, tolerance, 1)
        || check_row_sign(row, tolerance, -1)
        || check_row_sign(&reverse_row, tolerance, 1)
        || check_row_sign(&reverse_row, tolerance, -1);
}

#[test]
pub fn check_row_test() {
    let row: Vec<isize> = vec![1, 2, 1, 1];
    assert!(!check_row(&row, 1));

    let row: Vec<isize> = vec![1, 2, 1, 3];
    assert!(check_row(&row, 1));

    let row: Vec<isize> = vec![7, 6, 4, 2, 1];
    assert!(check_row(&row, 1));

    let row: Vec<isize> = vec![1, 3, 2, 4, 5];
    assert!(check_row(&row, 1));

    let row: Vec<isize> = vec![2, 1, 3, 4, 5];
    assert!(check_row(&row, 1));

    let row: Vec<isize> = vec![5, 1, 3, 4, 5];
    assert!(check_row(&row, 1));

    let row: Vec<isize> = vec![2, 7, 3, 4, 5];
    assert!(check_row(&row, 1));
}

#[test]
pub fn day2a() -> Result<(), Box<dyn Error>> {
    let matrix = read_input()?;

    let mut sum = 0;
    for row in matrix {
        if check_row(&row, 0) {
            sum += 1;
        }
    }

    return print_output("day2a".to_string(), sum);
}

#[test]
pub fn day2b() -> Result<(), Box<dyn Error>> {
    let matrix = read_input()?;

    let mut sum = 0;
    for row in matrix {
        if check_row(&row, 1) {
            sum += 1;
        }
    }

    return print_output("day2b".to_string(), sum);
}
