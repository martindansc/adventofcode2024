use crate::helpers::output::print_output;
use regex::Regex;
use std::{collections::VecDeque, error::Error, fs::read_to_string};

fn read_input() -> Result<Vec<VecDeque<String>>, Box<dyn Error>> {
    let input_file = "inputs/day4a.txt";

    let mut matrix: Vec<VecDeque<String>> = Vec::new();

    for line in read_to_string(input_file)?.lines() {
        let readed_line = line.to_string();
        let mut row: VecDeque<String> = readed_line
            .split_terminator("")
            .map(|x| x.to_string())
            .collect();

        row.pop_front();

        matrix.push(row);
    }

    return Ok(matrix);
}

// This funcion is not needed, but I wanted to implement an iterator just for fun
fn find_next_letter(
    matrix: Vec<VecDeque<String>>,
    letter: String,
) -> impl Iterator<Item = (isize, isize)> {
    let mut i = 0;

    let row_len = matrix[i].len();

    return std::iter::from_fn(move || {
        while i < matrix.len() * row_len {
            let row = &matrix[i / row_len];
            let j = i % row_len;
            if row[j] == letter {
                let result = Some(((i / row_len) as isize, j as isize));
                i += 1;
                return result;
            }

            i += 1;
        }

        return None;
    });
}

fn add_direction_to_position(
    position: (isize, isize),
    direction: &(isize, isize),
) -> (isize, isize) {
    let mut new_position = position.clone();
    new_position.0 += direction.0;
    new_position.1 += direction.1;
    return new_position;
}

fn check_word_direction(
    matrix: &Vec<VecDeque<String>>,
    mut position: (isize, isize),
    direction: &(isize, isize),
    word: &Vec<&str>,
) -> bool {
    let row_len = matrix[0].len();

    for step in 0..word.len() {
        if position.0 < 0 || position.1 < 0 {
            return false;
        }

        if position.0 >= matrix.len() as isize || position.1 >= row_len as isize {
            return false;
        }

        let character = &matrix[position.0 as usize][position.1 as usize];

        if character != word[step] {
            return false;
        }

        position = add_direction_to_position(position, direction);
    }

    return true;
}

fn check_word(matrix: &Vec<VecDeque<String>>, position: (isize, isize)) -> isize {
    let word = vec!["X", "M", "A", "S"];
    let mut sum = 0;
    for i in -1..2 {
        for j in -1..2 {
            if check_word_direction(matrix, position.clone(), &(i, j), &word) {
                sum += 1
            }
        }
    }

    return sum;
}

fn check_x(matrix: &Vec<VecDeque<String>>, position: (isize, isize)) -> isize {
    let mut sum = 0;

    let word = vec!["M", "A", "S"];

    for i in -1..2 {
        for j in -1..2 {
            if i == 0 || j == 0 {
                continue;
            }

            let direction = (i, j);
            let op_direction = (i * -1, j * -1);
            let new_position = add_direction_to_position(position, &op_direction);

            if check_word_direction(matrix, new_position, &direction, &word) {
                sum += 1
            }
        }
    }

    let mut extra = 0;
    if sum >= 2 {
        extra += 1;
    }

    return extra;
}

#[test]
pub fn day4a() -> Result<(), Box<dyn Error>> {
    let mut sum = 0;
    let matrix = read_input()?;

    for to_check in find_next_letter(matrix.clone(), "X".to_string()) {
        sum += check_word(&matrix, to_check);
    }

    return print_output("day4a".to_string(), sum);
}

#[test]
pub fn day4b() -> Result<(), Box<dyn Error>> {
    let mut sum = 0;
    let matrix = read_input()?;

    for to_check in find_next_letter(matrix.clone(), "A".to_string()) {
        sum += check_x(&matrix, to_check);
    }

    return print_output("day4b".to_string(), sum);
}
