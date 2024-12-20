use super::direction::Direction;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn add(&self, dir: &Direction) -> Self {
        Self {
            x: self.x + dir.i,
            y: self.y + dir.j,
        }
    }

    pub fn add_wrap(&self, dir: &Direction, max_x: isize, max_y: isize) -> Self {
        Self {
            x: (self.x + dir.i).rem_euclid(max_x),
            y: (self.y + dir.j).rem_euclid(max_y),
        }
    }

    pub fn direction_to(&self, position: &Position) -> Direction {
        return Direction {
            i: position.x - self.x,
            j: position.y - self.y,
        };
    }

    pub fn distance_to_manhattan(&self, p: &Position) -> isize {
        return (p.x - self.x).abs() + (p.y - self.y).abs();
    }
}
