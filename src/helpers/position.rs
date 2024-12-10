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
}
