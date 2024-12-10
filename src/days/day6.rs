use std::{collections::HashSet, error::Error};

use crate::helpers::{direction::Direction, map::Map, output::print_output, position::Position};

pub fn count_until_out_of_map(
    map: &Map<String>,
    starting_position: &Position,
) -> (bool, isize, HashSet<Position>) {
    let mut sum: isize = 1;
    let mut current_position: Position = starting_position.clone();
    let mut current_direction: Direction = Direction::NORTH();

    let mut visited: HashSet<Position> = HashSet::new();
    let mut visited_directions_positions: HashSet<(Direction, Position)> = HashSet::new();
    visited.insert(current_position.clone());

    loop {
        let next_position: Position = current_position.add(&current_direction);

        match map.get_cell(&next_position) {
            Some(value) => {
                if *value == "#".to_string() {
                    current_direction = current_direction.rotate_right(2);
                } else {
                    current_position = next_position;

                    let direction_position: (Direction, Position) =
                        (current_direction.clone(), current_position.clone());

                    if visited_directions_positions.contains(&direction_position) {
                        return (false, 0, visited);
                    }

                    if !visited.contains(&current_position) {
                        sum += 1;
                        visited.insert(current_position.clone());
                        visited_directions_positions
                            .insert((current_direction.clone(), current_position.clone()));
                    }
                }
            }
            None => return (true, sum, visited),
        }
    }
}

#[test]
pub fn day6a() -> Result<(), Box<dyn Error>> {
    let input_file = "day6a";
    let map: Map<String> = Map::read_input(&input_file)?;

    let starting_position: Position = map
        .clone()
        .find_next("^".to_string())
        .next()
        .expect("'^' should be present in input");

    let sum: (bool, isize, HashSet<Position>) = count_until_out_of_map(&map, &starting_position);

    return print_output("day6a".to_string(), sum.1);
}

#[test]
pub fn day6b() -> Result<(), Box<dyn Error>> {
    let input_file = "day6a";
    let mut map: Map<String> = Map::read_input(&input_file)?;

    let starting_position: Position = map
        .clone()
        .find_next("^".to_string())
        .next()
        .expect("'^' should be present in input");

    let initial: (bool, isize, HashSet<Position>) =
        count_until_out_of_map(&map, &starting_position);

    let mut sum: isize = 0;
    for possible_position in initial.2 {
        map.set_cell(&possible_position, &"#".to_string());
        let possible: (bool, isize, HashSet<Position>) =
            count_until_out_of_map(&map, &starting_position);

        if !possible.0 {
            sum += 1
        }

        map.set_cell(&possible_position, &".".to_string());
    }

    return print_output("day6b".to_string(), sum);
}
