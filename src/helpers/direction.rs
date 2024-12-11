use std::error::Error;

use super::{cmath::gcd, position::Position};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Direction {
    pub i: isize,
    pub j: isize,
}

#[allow(non_snake_case)]
impl Direction {
    pub fn NORTH() -> Self {
        return Self { i: -1, j: 0 };
    }

    pub fn NORTH_EAST() -> Self {
        return Self { i: 1, j: -1 };
    }

    pub fn EAST() -> Self {
        return Self { i: 0, j: 1 };
    }

    pub fn SOUTH_EAST() -> Self {
        return Self { i: 1, j: 1 };
    }

    pub fn SOUTH() -> Self {
        return Direction::NORTH().opposite();
    }

    pub fn SOUTH_WEST() -> Self {
        return Direction::NORTH_EAST().opposite();
    }

    pub fn WEST() -> Self {
        return Direction::EAST().opposite();
    }

    pub fn NORTH_WEAST() -> Self {
        return Direction::SOUTH_EAST().opposite();
    }

    pub fn ZERO() -> Self {
        return Self { i: 0, j: 0 };
    }

    pub fn all() -> Vec<Direction> {
        let ret: Vec<Direction> = vec![
            Direction::NORTH(),
            Direction::NORTH_EAST(),
            Direction::EAST(),
            Direction::SOUTH_EAST(),
            Direction::SOUTH(),
            Direction::SOUTH_WEST(),
            Direction::WEST(),
            Direction::NORTH_WEAST(),
            Direction::ZERO(),
        ];

        return ret;
    }

    pub fn rotate_right(&self, times: isize) -> Result<Self, Box<dyn Error>> {
        if self.i.abs() > 1 || self.j.abs() > 1 {
            return Err(Box::from(
                "Can't rotate a direction with values greater than 1",
            ));
        }

        let mut all: Vec<Direction> = Direction::all();
        all.pop();
        let i = all.iter().position(|e| *e == *self).unwrap();
        let new_index = (i as isize + times) % all.len() as isize;
        let next: Direction = all[new_index as usize].clone();
        return Ok(next);
    }

    pub fn opposite(&self) -> Self {
        return Self {
            i: self.i * -1,
            j: self.j * -1,
        };
    }

    pub fn simplify(&self) -> Self {
        if self.i == 0 {
            if self.j == 0 {
                return Direction::ZERO();
            } else if self.j > 0 {
                return Direction::EAST();
            } else {
                return Direction::WEST();
            }
        }

        if self.j == 0 {
            if self.i > 0 {
                return Direction::SOUTH();
            } else {
                return Direction::NORTH();
            }
        }

        let vgcd = gcd(self.i.abs() as usize, self.j.abs() as usize);

        return Self {
            i: self.i / vgcd as isize,
            j: self.j / vgcd as isize,
        };
    }
}

#[test]
pub fn test_rotate() {
    let mut current = Direction::NORTH();

    current = current.rotate_right(2).unwrap();
    assert_eq!(current, Direction::EAST());

    current = current.rotate_right(2).unwrap();
    assert_eq!(current, Direction::SOUTH());

    current = current.rotate_right(2).unwrap();
    assert_eq!(current, Direction::WEST());

    current = current.rotate_right(2).unwrap();
    assert_eq!(current, Direction::NORTH());
}

#[test]
pub fn test_simplify() {
    let mut current = Direction { i: 6, j: 8 };
    current = current.simplify();

    assert_eq!(current, Direction { i: 3, j: 4 });
}
