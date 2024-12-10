use crate::helpers::{direction::Direction, map::Map, output::print_output, position::Position};
use std::error::Error;

fn check_word_direction(
    map: &Map<String>,
    mut position: Position,
    direction: &Direction,
    word: &Vec<&str>,
) -> bool {
    for step in 0..word.len() {
        let character_opt = map.get_cell(&position);

        match character_opt {
            Some(character) => {
                if *character != word[step] {
                    return false;
                }
            }
            None => {
                return false;
            }
        }

        position = position.add(direction)
    }

    return true;
}

fn check_word(map: &Map<String>, position: Position) -> isize {
    let word = vec!["X", "M", "A", "S"];
    let mut sum = 0;

    for dir in Direction::all() {
        if check_word_direction(map, position.clone(), &dir, &word) {
            sum += 1
        }
    }

    return sum;
}

fn check_x(map: &Map<String>, position: Position) -> isize {
    let mut sum = 0;

    let word = vec!["M", "A", "S"];

    for dir in Direction::all() {
        if dir.i == 0 || dir.j == 0 {
            continue;
        }

        let op_direction = dir.opposite();
        let new_position = position.add(&op_direction);

        if check_word_direction(map, new_position, &dir, &word) {
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
    let input_file = "day4a";
    let mut sum = 0;
    let map: Map<String> = Map::read_input(&input_file)?;

    for to_check in map.clone().find_next("X".to_string()) {
        sum += check_word(&map, to_check);
    }

    return print_output("day4a".to_string(), sum);
}

#[test]
pub fn day4b() -> Result<(), Box<dyn Error>> {
    let input_file = "day4a";
    let mut sum = 0;
    let map: Map<String> = Map::read_input(input_file)?;

    for to_check in map.clone().find_next("A".to_string()) {
        sum += check_x(&map, to_check);
    }

    return print_output("day4b".to_string(), sum);
}
