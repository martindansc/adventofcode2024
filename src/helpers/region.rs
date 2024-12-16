use super::{direction::Direction, position::Position};
use std::isize;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Region {
    pub positions: Vec<Position>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Vertex {
    pub poistion: Position,
    pub direction: Direction,
}

impl Region {
    pub fn new(p: Position) -> Self {
        let mut positions = Vec::new();
        positions.push(p);
        return Self { positions };
    }

    pub fn add_to_region(&mut self, p: Position) {
        self.positions.push(p);
    }

    pub fn distance_to_manhattan(&self, p: &Position) -> isize {
        let mut min_distance = isize::MAX;
        let positions = &self.positions;
        for rp in positions {
            let d = rp.distance_to_manhattan(&p);
            if d < min_distance {
                min_distance = d;
            }
        }

        return min_distance;
    }

    pub fn area(&self) -> isize {
        return self.positions.len() as isize;
    }

    pub fn perimeter(&self) -> isize {
        let mut sum = 0;
        let positions = &self.positions;
        for rp in positions {
            sum += 4 - self.count_adjacent(&rp);
        }

        return sum;
    }

    pub fn has_adjacent_direction(&self, p: &Position, dir: &Direction) -> bool {
        let adj_position = p.add(dir);
        return self.positions.contains(&adj_position);
    }

    pub fn count_non_straight_vertices(&self, p: &Position) -> isize {
        let mut count = 0;

        //north-east
        if (!self.has_adjacent_direction(p, &Direction::NORTH())
            && !self.has_adjacent_direction(p, &Direction::EAST()))
            || (self.has_adjacent_direction(p, &Direction::NORTH())
                && self.has_adjacent_direction(p, &Direction::EAST()))
        {
            count += 1;
        } else if self.has_adjacent_direction(p, &Direction::NORTH_EAST()) {
            count += 1;
        }

        //north-west
        if (!self.has_adjacent_direction(p, &Direction::NORTH())
            && !self.has_adjacent_direction(p, &Direction::WEST()))
            || (self.has_adjacent_direction(p, &Direction::NORTH())
                && self.has_adjacent_direction(p, &Direction::WEST()))
        {
            count += 1;
        } else if self.has_adjacent_direction(p, &Direction::NORTH_WEST()) {
            count += 1;
        }

        //south-east
        if (!self.has_adjacent_direction(p, &Direction::SOUTH())
            && !self.has_adjacent_direction(p, &Direction::EAST()))
            || (self.has_adjacent_direction(p, &Direction::SOUTH())
                && self.has_adjacent_direction(p, &Direction::EAST()))
        {
            count += 1;
        } else if self.has_adjacent_direction(p, &Direction::SOUTH_EAST()) {
            count += 1;
        }

        //south-west
        if (!self.has_adjacent_direction(p, &Direction::SOUTH())
            && !self.has_adjacent_direction(p, &Direction::WEST()))
            || (self.has_adjacent_direction(p, &Direction::SOUTH())
                && self.has_adjacent_direction(p, &Direction::WEST()))
        {
            count += 1;
        } else if self.has_adjacent_direction(p, &Direction::SOUTH_WEST()) {
            count += 1;
        }

        return count;
    }

    pub fn stright_perimeter(&self) -> isize {
        let mut sum = 0;
        let mut to_substract = 0;
        let positions = &self.positions;
        for rp in positions {
            let vs = self.count_non_straight_vertices(rp);
            sum += vs;
            if vs == 3 {
                to_substract += 1;
            }
        }

        sum -= to_substract;

        return sum;
    }

    pub fn count_adjacent(&self, p: &Position) -> isize {
        let mut sum = 0;
        let positions = &self.positions;
        for rp in positions {
            if rp.distance_to_manhattan(p) == 1 {
                sum += 1;
            }
        }

        return sum;
    }

    pub fn union(&mut self, r: Region) {
        for rp in r.positions {
            self.add_to_region(rp);
        }
    }
}

#[test]
pub fn test_regions() {
    let mut matrix: Vec<Vec<Position>> = Vec::new();

    for i in 0..10 {
        let mut v: Vec<Position> = Vec::new();
        for j in 0..10 {
            v.push(Position { x: i, y: j });
        }

        matrix.push(v);
    }

    let mut region = Region::new(matrix[0][0].clone());

    region.add_to_region(matrix[1][0].clone());

    region.add_to_region(matrix[1][1].clone());

    assert_eq!(region.area(), 3);
    assert_eq!(region.perimeter(), 8);

    assert_eq!(region.count_non_straight_vertices(&matrix[0][0]), 3);
    assert_eq!(region.count_non_straight_vertices(&matrix[1][0]), 2);
    assert_eq!(region.count_non_straight_vertices(&matrix[1][1]), 3);

    assert_eq!(region.stright_perimeter(), 6);
}
