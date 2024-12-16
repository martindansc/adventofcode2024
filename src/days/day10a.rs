use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use crate::helpers::{direction::Direction, map::Map, output::print_output, position::Position};

pub fn add_initials(map: Map<isize>) -> HashMap<Position, HashSet<Position>> {
    let mut reachables: HashMap<Position, HashSet<Position>> = HashMap::new();

    for (p, value) in &map {
        let mut set = HashSet::new();
        if *value == 0 {
            set.insert(p.clone());
        }
        reachables.insert(p.clone(), set);
    }

    return reachables;
}

pub fn add_reachable(
    map: &Map<isize>,
    reachables: &mut HashMap<Position, HashSet<Position>>,
    number: isize,
) {
    let c_reachables = reachables.clone();
    let p_map = map.clone();

    for p in p_map.find_next(number) {
        let current_reachables = reachables.get_mut(&p).unwrap();
        for dir in Direction::manhattan() {
            let p_check = p.add(&dir);

            if let Some(val) = map.get_cell(&p_check) {
                if *val == number - 1 {
                    if let Some(previous) = c_reachables.get(&p_check) {
                        current_reachables.extend(previous.clone());
                    }
                }
            }
        }
    }
}

pub fn count_elements_at_number(
    map: Map<isize>,
    reachables: &mut HashMap<Position, HashSet<Position>>,
    number: isize,
) -> isize {
    let mut sum = 0;
    for p in map.find_next(number) {
        let r = reachables.get(&p).unwrap();
        sum += r.len();
    }
    return sum as isize;
}

#[test]
pub fn day10a() -> Result<(), Box<dyn Error>> {
    let input_file: &str = "day10a";
    let map: Map<isize> = Map::read_input_pop(&input_file, false)?;

    let mut reachables = add_initials(map.clone());

    for n in 1..10 {
        add_reachable(&map, &mut reachables, n);
    }

    let sum = count_elements_at_number(map.clone(), &mut reachables, 9);

    return print_output("day10a".to_string(), sum);
}
