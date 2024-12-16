use std::fmt::Debug;
use std::str::FromStr;
use std::{collections::VecDeque, error::Error, fs::read_to_string};

use super::position::Position;

#[derive(Clone, Debug)]
pub struct Map<T>
where
    T: FromStr + PartialEq + Clone + Debug,
{
    pub matrix: Vec<VecDeque<T>>,
}

impl<T: FromStr + PartialEq + Clone + Debug> Map<T> {
    pub fn read_input(test_name: &str) -> Result<Self, Box<dyn Error>> {
        return Self::read_input_pop(test_name, true);
    }

    pub fn read_input_pop(test_name: &str, need_pop: bool) -> Result<Self, Box<dyn Error>> {
        let input_file = "inputs/".to_owned() + test_name + ".txt";

        let mut matrix: Vec<VecDeque<T>> = Vec::new();

        for line in read_to_string(input_file)?.lines() {
            let readed_line = line.to_string();
            let mut row: VecDeque<T> = readed_line
                .split_terminator("")
                .flat_map(|x| x.parse::<T>())
                .collect();

            if need_pop {
                row.pop_front();
            }

            matrix.push(row);
        }

        return Ok(Self { matrix });
    }

    pub fn get_row_len(&self) -> isize {
        return self.matrix.len() as isize;
    }

    pub fn get_column_len(&self) -> isize {
        if self.get_row_len() == 0 {
            return 0;
        }

        return self.matrix[0].len() as isize;
    }

    pub fn out_of_map(&self, position: &Position) -> bool {
        if self.get_column_len() <= position.y {
            return true;
        }

        if self.get_row_len() <= position.x {
            return true;
        }

        if position.x < 0 || position.y < 0 {
            return true;
        }

        return false;
    }

    pub fn get_cell(&self, position: &Position) -> Option<&T> {
        if self.out_of_map(position) {
            return None;
        }

        return Some(&self.matrix[position.x as usize][position.y as usize]);
    }

    pub fn get_position_i(&self, i: isize) -> Position {
        let column_len = self.get_column_len();

        let position = Position {
            x: i / column_len,
            y: i % column_len,
        };

        return position;
    }

    pub fn get_cell_i(&self, i: isize) -> Option<&T> {
        let position = self.get_position_i(i);
        return self.get_cell(&position);
    }

    pub fn set_cell(&mut self, position: &Position, value: &T) {
        self.matrix[position.x as usize][position.y as usize] = value.clone();
    }

    pub fn find_next(self: Map<T>, item: T) -> impl Iterator<Item = Position> {
        let mut i = 0;

        let row_len = self.get_row_len();
        let column_len = self.get_column_len();

        return std::iter::from_fn(move || {
            while i < column_len * row_len {
                let position = self.get_position_i(i);

                let value = self.get_cell(&position).unwrap();
                if *value == item {
                    let result = Some(position);
                    i += 1;
                    return result;
                }

                i += 1;
            }

            return None;
        });
    }
}

pub struct MapIterator<'a, T: FromStr + PartialEq + Clone + Debug> {
    map: &'a Map<T>,
    index: isize,
}

impl<'a, T: FromStr + PartialEq + Clone + Debug> Iterator for MapIterator<'a, T> {
    type Item = (Position, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        let next_position = self.map.get_position_i(self.index);
        let next_value = self.map.get_cell(&next_position)?;
        self.index += 1;
        return Some((next_position, next_value));
    }
}

impl<'a, T: FromStr + PartialEq + Clone + Debug> IntoIterator for &'a Map<T> {
    type Item = (Position, &'a T);
    type IntoIter = MapIterator<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        return MapIterator {
            map: self,
            index: 0,
        };
    }
}
