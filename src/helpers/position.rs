use super::direction::Direction;

#[derive(Clone)]
pub struct Position {
    pub x: isize,
    pub y: isize,
}

impl Position {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn add(&self, dir: &Direction) -> Self {
        Self {
            x: self.x + dir.i,
            y: self.y + dir.j,
        }
    }
}
