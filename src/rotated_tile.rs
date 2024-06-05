use crate::direction::Direction;
use crate::feature::Feature;
use crate::rotation::Rotation;
use crate::tile::Tile;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct RotatedTile {
    tile: Tile,
    rotation: Rotation,
}

impl RotatedTile {
    pub fn new(tile: Tile, rotation: Rotation) -> Self {
        Self { tile, rotation }
    }

    pub fn feature(&self, side: Direction) -> Feature {
        use Feature::City as C;
        use Feature::Forest as F;
        use Feature::Road as R;

        let features = match self.tile {
            Tile::CCCCS => [C, C, C, C],
            Tile::CCCF | Tile::CCCFS => [C, C, C, F],
            Tile::CCCR | Tile::CCCRS => [C, C, C, R],
            Tile::CCFF | Tile::CCFFS | Tile::CCFF2 => [C, C, F, F],
            Tile::CCRR | Tile::CCRRS => [C, C, R, R],
            Tile::CFCF | Tile::CFCFS | Tile::CFCF2 => [C, F, C, F],
            Tile::CFFF => [C, F, F, F],
            Tile::CFRR => [C, F, R, R],
            Tile::CRFR => [C, R, F, R],
            Tile::CRRF => [C, R, R, F],
            Tile::CRRR => [C, R, R, R],
            Tile::FFFF => [F, F, F, F],
            Tile::FFFR => [F, F, F, R],
            Tile::FFRR => [F, F, R, R],
            Tile::FRFR => [F, R, F, R],
            Tile::FRRR => [F, R, R, R],
            Tile::RRRR => [R, R, R, R],
        };

        let index = (side as usize + (4 - self.rotation as usize)) % 4;

        features[index]
    }

    /// Predicate returning whether `other` can connect to the given `side` of `self`.
    pub fn connects(&self, other: &Self, side: Direction) -> bool {
        self.feature(side) == other.feature(side.rev())
    }

    pub fn tile(&self) -> &Tile {
        &self.tile
    }
}

#[macro_export]
macro_rules! tile {
    ($tile:ident, $rotation:ident) => {{
        $crate::rotated_tile::RotatedTile {
            tile: $crate::tile::Tile::$tile,
            rotation: $crate::rotation::Rotation::$rotation,
        }
    }};
}

#[cfg(test)]
mod tests {
    use crate::direction::Direction;

    #[test]
    fn small_city() {
        let top = tile!(CFFF, D180);
        let bottom = tile!(CFFF, D0);
        assert!(top.connects(&bottom, Direction::South));
    }

    #[test]
    fn small_city_wrong_rotation() {
        let top = tile!(CFFF, D0);
        let bottom = tile!(CFFF, D0);
        assert!(!top.connects(&bottom, Direction::South));
    }
}
