use enum_iterator::Sequence;

/// Enumerated Tiles for BaseGame_C1 variant of Carcassonne.
/// Ordered, Rotated, Named and Weighted as https://wikicarpedia.com/car/Carcassonne_Tile_List/en.
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Copy, Clone, Sequence)]
pub enum Tile {
    /// city-city-city-city-shield
    CCCCS,
    /// city-city-city-forest
    CCCF,
    /// city-city-city-forest-shield
    CCCFS,
    /// city-city-city-road
    CCCR,
    /// city-city-city-road-shield
    CCCRS,
    /// city-city-forest-forest
    CCFF,
    /// city-city-forest-forest-shield
    CCFFS,
    /// city-city-forest-forest (alternative)
    CCFF2,
    /// city-city-road-road
    CCRR,
    /// city-city-road-road-shield
    CCRRS,
    /// city-forest-city-forest
    CFCF,
    /// city-fores-city-forest-shield
    CFCFS,
    /// city-forest-city-forest (alternative)
    CFCF2,
    /// city-forest-forest-forest
    CFFF,
    /// city-forest-road-road
    CFRR,
    /// city-road-forest-road
    CRFR,
    /// city-road-road-forest
    CRRF,
    /// city-road-road-road
    CRRR,
    /// forest-forest-forest-forest
    FFFF,
    /// forest-forest-forest-road
    FFFR,
    /// forest-forest-road-road
    FFRR,
    /// forest-road-forest-road
    FRFR,
    /// forest-road-road-road
    FRRR,
    /// road-road-road-road
    RRRR,
}

impl Tile {
    pub fn total_weight() -> usize {
        enum_iterator::all::<Tile>().map(|tile| tile.weight()).sum()
    }

    pub fn weight(&self) -> usize {
        match self {
            Tile::CCCCS => 1,
            Tile::CCCF => 3,
            Tile::CCCFS => 1,
            Tile::CCCR => 1,
            Tile::CCCRS => 2,
            Tile::CCFF => 3,
            Tile::CCFFS => 2,
            Tile::CCFF2 => 2,
            Tile::CCRR => 3,
            Tile::CCRRS => 2,
            Tile::CFCF => 1,
            Tile::CFCFS => 2,
            Tile::CFCF2 => 3,
            Tile::CFFF => 5,
            Tile::CFRR => 3,
            Tile::CRFR => 3,
            Tile::CRRF => 3,
            Tile::CRRR => 3,
            Tile::FFFF => 4,
            Tile::FFFR => 2,
            Tile::FFRR => 9,
            Tile::FRFR => 8,
            Tile::FRRR => 4,
            Tile::RRRR => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn base_total_tiles() {
        todo!("count number of tiles")
        // assert_eq!(Tile::total_weight(), 71);
    }
}
