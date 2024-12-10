use crate::helpers::output::print_output;
use std::{collections::VecDeque, error::Error, fs::read_to_string};

fn read_input() -> Result<Vec<(usize, VecDeque<usize>)>, Box<dyn Error>> {
    let input_file = "inputs/day7a.txt";

    let mut matrix: Vec<(usize, VecDeque<usize>)> = Vec::new();

    for line in read_to_string(input_file)?.lines() {
        let readed_line = line.to_string();
        let mut row: VecDeque<usize> = readed_line
            .split_terminator(" ")
            .map(|x| {
                return x
                    .replace(":", "")
                    .parse::<usize>()
                    .expect("Input should be correct");
            })
            .collect();

        let value = row.pop_front().expect("Input should be correct");

        matrix.push((value, row));
    }

    return Ok(matrix);
}

pub fn concat_values(val1: usize, val2: usize) -> usize {
    return (val1.to_string() + &val2.to_string()).parse().unwrap();
}

pub fn check(
    target: usize,
    nums: &VecDeque<usize>,
    i: usize,
    current: usize,
    concat_op: bool,
) -> bool {
    if i == nums.len() {
        return target == current;
    }

    let value = nums[i];

    return check(target, nums, i + 1, current + value, concat_op)
        || check(target, nums, i + 1, current * value, concat_op)
        || (concat_op
            && check(
                target,
                nums,
                i + 1,
                concat_values(current, value),
                concat_op,
            ));
}

#[test]
pub fn day7a() -> Result<(), Box<dyn Error>> {
    let input = read_input()?;

    let mut sum = 0;

    for row in input {
        if check(row.0, &row.1, 0, 0, false) {
            sum += row.0;
        }
    }

    return print_output("day7a".to_string(), sum as isize);
}

#[test]
pub fn day7b() -> Result<(), Box<dyn Error>> {
    let input = read_input()?;

    let mut sum = 0;

    for row in input {
        if check(row.0, &row.1, 0, 0, true) {
            sum += row.0;
        }
    }

    return print_output("day7b".to_string(), sum as isize);
}
