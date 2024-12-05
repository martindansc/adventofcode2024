use crate::helpers::output::print_output;
use std::{collections::HashMap, error::Error, fs::read_to_string};

fn read_input() -> Result<(HashMap<isize, Vec<isize>>, Vec<Vec<isize>>), Box<dyn Error>> {
    let input_file = "inputs/day5a.txt";
    let input = read_to_string(input_file)?;
    let mut lines = input.lines();
    let mut line = lines
        .next()
        .expect("Input should have at least empty one line");

    let mut relative_orders: HashMap<isize, Vec<isize>> = HashMap::new();
    let mut updates: Vec<Vec<isize>> = Vec::new();

    while line != "" {
        let readed_line = line.to_string();
        let row: Vec<isize> = readed_line
            .split_terminator("|")
            .map(|x| x.parse::<isize>().expect("Input should be correct"))
            .collect();

        let first: isize = row[0];
        let second: isize = row[1];

        if relative_orders.contains_key(&second) {
            relative_orders.get_mut(&second).unwrap().push(first);
        } else {
            relative_orders.insert(second, vec![first]);
        }

        line = lines
            .next()
            .expect("Input should have at least one empty line");
    }

    while let Some(line) = lines.next() {
        let readed_line = line.to_string();
        let row: Vec<isize> = readed_line
            .split_terminator(",")
            .map(|x| x.parse::<isize>().expect("Input should be correct"))
            .collect();

        updates.push(row);
    }

    return Ok((relative_orders, updates));
}

pub fn check_update(update: &Vec<isize>, relative_orders: &HashMap<isize, Vec<isize>>) -> bool {
    for i in 0..update.len() {
        let num = update[i];
        let order_opt: Option<&Vec<isize>> = relative_orders.get(&num);

        if order_opt.is_none() {
            continue;
        }

        let order = order_opt.expect("This should exists");

        for j in i + 1..update.len() {
            let num_2: isize = update[j];

            if order.contains(&num_2) {
                return false;
            }
        }
    }

    return true;
}

pub fn repair_update(
    update: &Vec<isize>,
    relative_orders: &HashMap<isize, Vec<isize>>,
) -> Vec<isize> {
    let mut improved = update.clone();
    for i in 0..update.len() {
        let num = update[i];
        let order_opt: Option<&Vec<isize>> = relative_orders.get(&num);

        if order_opt.is_none() {
            continue;
        }

        let order = order_opt.expect("This should exists");

        for j in i + 1..update.len() {
            let num_2: isize = update[j];

            if order.contains(&num_2) {
                improved[i] = num_2;
                improved[j] = num;
                return improved;
            }
        }
    }

    return improved;
}

#[test]
pub fn day5a() -> Result<(), Box<dyn Error>> {
    let (relative_orders, updates) = read_input()?;

    let mut sum = 0;

    for update in updates {
        if check_update(&update, &relative_orders) {
            let i = update.len() / 2;
            sum += update[i];
        }
    }

    return print_output("day5a".to_string(), sum);
}

#[test]
pub fn day5b() -> Result<(), Box<dyn Error>> {
    let (relative_orders, updates) = read_input()?;

    let mut sum = 0;

    for update in updates {
        if !check_update(&update, &relative_orders) {
            let mut repaired = repair_update(&update, &relative_orders);
            while !check_update(&repaired, &relative_orders) {
                repaired = repair_update(&repaired, &relative_orders);
            }

            let i = repaired.len() / 2;
            sum += repaired[i];
        }
    }

    return print_output("day5b".to_string(), sum);
}
