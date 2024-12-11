use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use crate::helpers::{direction::Direction, map::Map, output::print_output, position::Position};

pub fn get_different_letters(map: &Map<String>) -> HashMap<String, Vec<Position>> {
    let mut ret: HashMap<String, Vec<Position>> = HashMap::new();

    for (position, value) in map {
        if *value == ".".to_string() {
            continue;
        }

        if !ret.contains_key(value) {
            ret.insert(value.to_owned(), Vec::new());
        }

        let letters_positions = ret.get_mut(value).unwrap();
        letters_positions.push(position);
    }

    return ret;
}

pub fn calculate_influence_letter(
    map: &Map<String>,
    letter_positions: &Vec<Position>,
) -> HashSet<Position> {
    let mut bag: HashSet<Position> = HashSet::new();

    for i in 0..letter_positions.len() {
        let p1: &Position = &letter_positions[i];
        for j in i + 1..letter_positions.len() {
            let p2: &Position = &letter_positions[j];

            let dir: Direction = p1.direction_to(p2);

            let p1_check: Position = p1.add(&dir.opposite());
            if !map.out_of_map(&p1_check) {
                bag.insert(p1_check);
            }

            let p2_check: Position = p2.add(&dir);
            if !map.out_of_map(&p2_check) {
                bag.insert(p2_check);
            }
        }
    }

    return bag;
}

pub fn calculate_influence_letter_v2(
    map: &Map<String>,
    letter_positions: &Vec<Position>,
) -> HashSet<Position> {
    let mut bag: HashSet<Position> = HashSet::new();

    for i in 0..letter_positions.len() {
        let p1: &Position = &letter_positions[i];
        for j in i + 1..letter_positions.len() {
            let p2: &Position = &letter_positions[j];

            let dir: Direction = p1.direction_to(p2).simplify();

            let mut p1_check: Position = p1.add(&dir.opposite());
            while !map.out_of_map(&p1_check) {
                bag.insert(p1_check.clone());
                p1_check = p1_check.add(&dir.opposite());
            }

            let mut p2_check: Position = p2.add(&dir);
            while !map.out_of_map(&p2_check) {
                bag.insert(p2_check.clone());
                p2_check = p2_check.add(&dir);
            }

            let mut p1_check: Position = p1.clone();
            while !map.out_of_map(&p1_check) {
                bag.insert(p1_check.clone());
                p1_check = p1_check.add(&dir);
            }
        }
    }

    return bag;
}

#[test]
pub fn day8a() -> Result<(), Box<dyn Error>> {
    let input_file: &str = "day8a";
    let map: Map<String> = Map::read_input(&input_file)?;

    let letters_groups: HashMap<String, Vec<Position>> = get_different_letters(&map);

    let mut nodes: HashSet<Position> = HashSet::new();
    for letter_group in letters_groups {
        let new_nodes: HashSet<Position> = calculate_influence_letter(&map, &letter_group.1);
        nodes.extend(new_nodes);
    }

    return print_output("day8a".to_string(), nodes.len() as isize);
}

#[test]
pub fn day8b() -> Result<(), Box<dyn Error>> {
    let input_file: &str = "day8a";
    let map: Map<String> = Map::read_input(&input_file)?;

    let letters_groups: HashMap<String, Vec<Position>> = get_different_letters(&map);

    let mut nodes: HashSet<Position> = HashSet::new();
    for letter_group in letters_groups {
        let new_nodes: HashSet<Position> = calculate_influence_letter_v2(&map, &letter_group.1);
        nodes.extend(new_nodes);
    }

    let num: isize = nodes.len() as isize;

    return print_output("day8a".to_string(), num);
}
