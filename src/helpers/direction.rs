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

    pub fn rotate_right(&self, times: isize) -> Self {
        let mut all: Vec<Direction> = Direction::all();
        all.pop();
        let i = all.iter().position(|e| *e == *self).unwrap();
        let new_index = (i as isize + times) % all.len() as isize;
        let next: Direction = all[new_index as usize].clone();
        return next;
    }

    pub fn opposite(&self) -> Self {
        return Self {
            i: self.i * -1,
            j: self.j * -1,
        };
    }
}

#[test]
pub fn test_rotate() {
    let mut current = Direction::NORTH();

    current = current.rotate_right(2);
    assert_eq!(current, Direction::EAST());

    current = current.rotate_right(2);
    assert_eq!(current, Direction::SOUTH());

    current = current.rotate_right(2);
    assert_eq!(current, Direction::WEST());

    current = current.rotate_right(2);
    assert_eq!(current, Direction::NORTH());
}
