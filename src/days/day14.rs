use regex::Regex;

use crate::helpers::{direction::Direction, output::print_output, position::Position};
use std::{error::Error, fs::read_to_string};

const N_ROWS: isize = 101;
const N_COLUMNS: isize = 103;

fn read_input() -> Result<Vec<(Position, Direction)>, Box<dyn Error>> {
    let input_file = "inputs/day14a.txt";

    let mut res: Vec<(Position, Direction)> = Vec::new();

    let re: Regex = Regex::new(r"(-?[0-9]+),(-?[0-9]+)").unwrap();

    let binding = read_to_string(input_file)?;
    let lines: Vec<&str> = binding.lines().collect();

    for i in 0..lines.len() {
        let mut captured = re.captures_iter(lines[i]);

        let c_position = captured.next().unwrap();
        let pos = Position {
            x: c_position.get(1).unwrap().as_str().parse().unwrap(),
            y: c_position.get(2).unwrap().as_str().parse().unwrap(),
        };

        let c_direction = captured.next().unwrap();
        let dir = Direction {
            i: c_direction.get(1).unwrap().as_str().parse().unwrap(),
            j: c_direction.get(2).unwrap().as_str().parse().unwrap(),
        };

        res.push((pos, dir));
    }

    return Ok(res);
}

pub fn advance_time(values: &Vec<(Position, Direction)>) -> Vec<(Position, Direction)> {
    let mut new_vec: Vec<(Position, Direction)> = Vec::new();

    for (p, d) in values {
        let new_v = p.add_wrap(&d, N_ROWS, N_COLUMNS);
        new_vec.push((new_v, d.clone()));
    }

    return new_vec;
}

#[test]
pub fn day14a() -> Result<(), Box<dyn Error>> {
    let mut values = read_input()?;

    for _i in 0..100 {
        values = advance_time(&values);
    }

    let mut sum: Vec<isize> = vec![0, 0, 0, 0];
    for (p, _d) in values {
        if p.x < N_ROWS / 2 {
            if p.y < N_COLUMNS / 2 {
                sum[0] += 1;
            }
            if p.y > N_COLUMNS / 2 {
                sum[1] += 1;
            }
        }

        if p.x > N_ROWS / 2 {
            if p.y < N_COLUMNS / 2 {
                sum[2] += 1;
            }
            if p.y > N_COLUMNS / 2 {
                sum[3] += 1;
            }
        }
    }

    return print_output("day14a".to_string(), sum[0] * sum[1] * sum[2] * sum[3]);
}

fn count_neighbours(values: &Vec<(Position, Direction)>) -> isize {
    let mut sum = 0;
    for i in 0..values.len() {
        let p1 = values[i].0.clone();
        for j in i + 1..values.len() {
            let p2 = values[j].0.clone();

            if p1.distance_to_manhattan(&p2) == 1 {
                sum += 1;
                break;
            }
        }
    }

    return sum;
}

#[test]
pub fn day14b() -> Result<(), Box<dyn Error>> {
    let mut values = read_input()?;

    let mut last_id = 0;
    let mut max = 0;
    for _i in 1..10000 {
        values = advance_time(&values);
        let new_max = count_neighbours(&values);

        if new_max > max {
            max = new_max;
            last_id = _i;
        }
    }

    return print_output("day14a".to_string(), last_id);
}
