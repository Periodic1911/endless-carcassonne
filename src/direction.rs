use crate::map::Point2D;
use enum_iterator::Sequence;

/// Cardinal directions.
#[derive(Debug, Copy, Clone, Sequence)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl From<Direction> for usize {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => 0,
            Direction::East => 1,
            Direction::South => 2,
            Direction::West => 3,
        }
    }
}

impl Direction {
    pub fn rev(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    pub fn step(&self) -> Point2D {
        match self {
            Direction::North => (0, 1),
            Direction::East => (1, 0),
            Direction::South => (0, -1),
            Direction::West => (-1, 0),
        }
    }
}
