use core::panic;
use std::{collections::VecDeque, error::Error, fs::read_to_string};

use crate::helpers::{direction::Direction, map::Map, output::print_output, position::Position};

pub fn read_instructions() -> Result<VecDeque<String>, Box<dyn Error>> {
    let test_name = "inputs/day15b.txt";

    let mut input: VecDeque<String> = VecDeque::new();

    for line in read_to_string(test_name)?.lines() {
        let readed_line = line.to_string();
        let mut readed_line_split: VecDeque<&str> = readed_line.split_terminator("").collect();

        readed_line_split.pop_front();

        for value in readed_line_split {
            input.push_back(value.to_string());
        }
    }

    return Ok(input);
}

pub fn transform_map(map: &Map<String>) -> Map<String> {
    let mut new_map: Map<String> =
        Map::new(map.get_row_len(), map.get_column_len() * 2, ".".to_string());
    for cell in map {
        let mut new_cell = cell.0.clone();
        new_cell.y *= 2;
        if *cell.1 == "O".to_string() {
            new_map.set_cell(&new_cell, &"[".to_string());
        } else {
            new_map.set_cell(&new_cell, &cell.1);
        }

        if *cell.1 == "@".to_string() {
            continue;
        }

        new_cell.y += 1;

        if *cell.1 == "O".to_string() {
            new_map.set_cell(&new_cell, &"]".to_string());
        } else {
            new_map.set_cell(&new_cell, &cell.1);
        }
    }

    return new_map;
}

pub fn move_to(map: &mut Map<String>, position: &Position, dir: &Direction) -> bool {
    let next_position = position.add(&dir);
    let next_cell = map.get_cell(&next_position);

    let current_value = map.get_cell(&position).unwrap().clone();

    if let Some(cell) = next_cell {
        if *cell == "#" {
            return false;
        } else if *cell == "." {
            map.set_cell(&next_position, &current_value);
            map.set_cell(position, &".".to_string());
            return true;
        } else if *cell == "O"
            || ((*dir == Direction::EAST() || *dir == Direction::WEST())
                && (*cell == "[" || *cell == "]"))
        {
            if move_to(map, &next_position, dir) {
                map.set_cell(&next_position, &current_value);
                map.set_cell(position, &".".to_string());
                return true;
            }
        } else if *cell == "[" {
            let save_map = map.clone();
            if move_to(map, &next_position, dir) {
                if move_to(map, &next_position.add(&Direction::EAST()), dir) {
                    map.set_cell(&next_position, &current_value);
                    map.set_cell(position, &".".to_string());
                    return true;
                }
            }
            *map = save_map;
        } else if *cell == "]" {
            let save_map = map.clone();
            if move_to(map, &next_position, dir) {
                if move_to(map, &next_position.add(&Direction::WEST()), dir) {
                    map.set_cell(&next_position, &current_value);
                    map.set_cell(position, &".".to_string());
                    return true;
                }
            }
            *map = save_map;
        }
    }

    return false;
}

pub fn iterate_one(
    map: &mut Map<String>,
    robot_position: &mut Position,
    instructions: &mut VecDeque<String>,
) {
    let next_instruction = instructions.pop_front().unwrap();

    match next_instruction.as_str() {
        "^" => {
            if move_to(map, robot_position, &Direction::NORTH()) {
                *robot_position = robot_position.add(&Direction::NORTH());
            };
        }
        ">" => {
            if move_to(map, robot_position, &Direction::EAST()) {
                *robot_position = robot_position.add(&Direction::EAST());
            }
        }
        "v" => {
            if move_to(map, robot_position, &Direction::SOUTH()) {
                *robot_position = robot_position.add(&Direction::SOUTH());
            }
        }
        "<" => {
            if move_to(map, robot_position, &Direction::WEST()) {
                *robot_position = robot_position.add(&Direction::WEST());
            }
        }
        _ => panic!(),
    };
}

#[test]
pub fn day15a() -> Result<(), Box<dyn Error>> {
    let test_name = "day15a";
    let mut map: Map<String> = Map::read_input(test_name)?;
    let mut robot_position: Position = map.clone().find_next("@".to_string()).next().unwrap();

    let mut instructions = read_instructions()?;

    while !instructions.is_empty() {
        iterate_one(&mut map, &mut robot_position, &mut instructions);
    }

    let mut sum = 0;
    for b in map.find_next("O".to_string()) {
        sum += b.x * 100 + b.y;
    }

    return print_output("day15a".to_string(), sum);
}

#[test]
pub fn day15b() -> Result<(), Box<dyn Error>> {
    let test_name = "day15a";
    let mut map: Map<String> = Map::read_input(test_name)?;
    map = transform_map(&map);
    let mut robot_position: Position = map.clone().find_next("@".to_string()).next().unwrap();

    let mut instructions = read_instructions()?;

    while !instructions.is_empty() {
        iterate_one(&mut map, &mut robot_position, &mut instructions);
    }

    let mut sum = 0;
    for b in map.find_next("[".to_string()) {
        sum += b.x * 100 + b.y;
    }

    return print_output("day15b".to_string(), sum);
}
