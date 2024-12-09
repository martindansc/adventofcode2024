pub struct Direction {
    pub i: isize,
    pub j: isize,
}

impl Direction {
    pub fn new(i: isize, j: isize) -> Self {
        Self { i, j }
    }

    pub fn all() -> Vec<Direction> {
        let mut ret: Vec<Direction> = Vec::new();

        for i in -1..2 {
            for j in -1..2 {
                let dir = Direction::new(i, j);
                ret.push(dir);
            }
        }

        return ret;
    }

    pub fn opposite(&self) -> Self {
        Self {
            i: self.i * -1,
            j: self.j * -1,
        }
    }
}
