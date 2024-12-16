use regex::Regex;

use eqsolver::{
    multivariable::MultiVarNewtonFD,
    nalgebra::{self, Matrix2x1, Vector2},
};
use nalgebra::{vector, Matrix2};

use crate::helpers::output::print_output;
use std::{error::Error, fs::read_to_string};

fn read_input(
    add: isize,
) -> Result<(Vec<(isize, isize, isize, isize, isize, isize)>), Box<dyn Error>> {
    let input_file = "inputs/day13a.txt";

    let mut res: Vec<(isize, isize, isize, isize, isize, isize)> = Vec::new();

    let re: Regex = Regex::new(r"X\+([0-9]+).*Y\+([0-9]+)").unwrap();
    let re2: Regex = Regex::new(r"X\=([0-9]+).*Y\=([0-9]+)").unwrap();

    let binding = read_to_string(input_file)?;
    let lines: Vec<&str> = binding.lines().collect();

    for i in 0..lines.len() / 4 + 1 {
        let captured_1 = re.captures(lines[i * 4]).unwrap();
        let x1: isize = captured_1.get(1).unwrap().as_str().to_string().parse()?;
        let y1: isize = captured_1.get(2).unwrap().as_str().to_string().parse()?;

        let captured_2 = re.captures(lines[i * 4 + 1]).unwrap();
        let x2: isize = captured_2.get(1).unwrap().as_str().to_string().parse()?;
        let y2: isize = captured_2.get(2).unwrap().as_str().to_string().parse()?;

        let captured_3 = re2.captures(lines[i * 4 + 2]).unwrap();
        let x3: isize = captured_3.get(1).unwrap().as_str().to_string().parse()?;
        let y3: isize = captured_3.get(2).unwrap().as_str().to_string().parse()?;

        res.push((x1, y1, x2, y2, x3 + add, y3 + add));
    }

    return Ok(res);
}

pub fn solve(formula: (isize, isize, isize, isize, isize, isize)) -> Option<(isize, isize)> {
    let (x1, y1, x2, y2, x3, y3) = formula;
    /* let f = |v: Vector2<f64>| {
        vector![
            x1 as f64 * v[0] + x2 as f64 * v[1] - x3 as f64,
            y1 as f64 * v[0] + y2 as f64 * v[1] - y3 as f64,
        ]
    };

    let solution = MultiVarNewtonFD::new(f).solve(vector![50., 50.]);

    println!("sol: {:?}", solution);

    let a = (*solution.as_ref().unwrap().get(0).unwrap()).round() as isize;
    let b = (*solution.as_ref().unwrap().get(1).unwrap()).round() as isize; */

    let matrix = Matrix2::new(x1 as f64, x2 as f64, y1 as f64, y2 as f64);

    let inverse = matrix.try_inverse().unwrap();

    let matrix_res = Matrix2x1::new(x3 as f64, y3 as f64);
    let res = inverse * matrix_res;

    let a = res.get(0).unwrap().round() as isize;
    let b = res.get(1).unwrap().round() as isize;

    if a * x1 + b * x2 != x3 || a * y1 + b * y2 != y3 {
        return None;
    }

    if a < 0 || b < 0 {
        return None;
    }

    return Some((a, b));
}

#[test]
pub fn day13a() -> Result<(), Box<dyn Error>> {
    let values = read_input(0).unwrap();
    println!("{:?}", values);

    let mut sum = 0;
    for v in values {
        let solution = solve(v);
        if let Some(val) = solution {
            sum += val.0 * 3 + val.1;
        }

        println!("{:?}", solution);
    }

    return print_output("day13a".to_string(), sum);
}

#[test]
pub fn day13b() -> Result<(), Box<dyn Error>> {
    let values = read_input(10000000000000).unwrap();
    println!("{:?}", values);

    let mut sum = 0;
    for v in values {
        let solution = solve(v);
        if let Some(val) = solution {
            sum += val.0 * 3 + val.1;
        }

        println!("{:?}", solution);
    }

    return print_output("day13b".to_string(), sum);
}
