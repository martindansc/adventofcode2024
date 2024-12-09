use crate::helpers::{direction::Direction, output::print_output, position::Position};
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
) -> impl Iterator<Item = Position> {
    let mut i = 0;

    let row_len = matrix[i].len();

    return std::iter::from_fn(move || {
        while i < matrix.len() * row_len {
            let row = &matrix[i / row_len];
            let j = i % row_len;
            if row[j] == letter {
                let result = Some(Position::new((i / row_len) as isize, j as isize));
                i += 1;
                return result;
            }

            i += 1;
        }

        return None;
    });
}

fn check_word_direction(
    matrix: &Vec<VecDeque<String>>,
    mut position: Position,
    direction: &Direction,
    word: &Vec<&str>,
) -> bool {
    let row_len = matrix[0].len();

    for step in 0..word.len() {
        if position.x < 0 || position.y < 0 {
            return false;
        }

        if position.x >= matrix.len() as isize || position.y >= row_len as isize {
            return false;
        }

        let character = &matrix[position.x as usize][position.y as usize];

        if character != word[step] {
            return false;
        }

        position = position.add(direction)
    }

    return true;
}

fn check_word(matrix: &Vec<VecDeque<String>>, position: Position) -> isize {
    let word = vec!["X", "M", "A", "S"];
    let mut sum = 0;

    for dir in Direction::all() {
        if check_word_direction(matrix, position.clone(), &dir, &word) {
            sum += 1
        }
    }

    return sum;
}

fn check_x(matrix: &Vec<VecDeque<String>>, position: Position) -> isize {
    let mut sum = 0;

    let word = vec!["M", "A", "S"];

    for dir in Direction::all() {
        if dir.i == 0 || dir.j == 0 {
            continue;
        }

        let op_direction = dir.opposite();
        let new_position = position.add(&op_direction);

        if check_word_direction(matrix, new_position, &dir, &word) {
            sum += 1
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
