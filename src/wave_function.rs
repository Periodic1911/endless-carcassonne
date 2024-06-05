use crate::direction::Direction;
use crate::fixed_map::FixedMap2D;
use crate::rotated_tile::RotatedTile;
use crate::rotation::Rotation;
use crate::tile::Tile;
use crate::Args;
use itertools::Itertools;
use rand::distributions::WeightedIndex;
use rand::{Rng, RngCore};
use std::collections::HashSet;

pub type TileMap = FixedMap2D<Option<RotatedTile>>;

pub fn generate<R: RngCore>(args: Args, rng: &mut R) -> Option<TileMap> {
    // 1. Initialize map.
    let mut tile_map = TileMap::default();

    // 2. Compute the initial candidates.
    let candidates = enum_iterator::all::<Tile>()
        .cartesian_product(enum_iterator::all::<Rotation>())
        .map(|(tile, rotation)| RotatedTile::new(tile, rotation))
        .collect();

    // 3. Populate the candidate map.
    let mut candidate_map = FixedMap2D::<HashSet<RotatedTile>>::default();
    candidate_map.fill(candidates);

    // 4. Choose random starting point.
    let start = tile_map.sample_point(rng);
    let mut queue = HashSet::from([start]);

    // 7. Repeat while the map is not filled.
    'outer: while let Some(&p @ (x, y)) = queue.iter().min_by_key(|&&p| entropy(&candidate_map[p]))
    {
        // 5. Collapse cell with the least entropy.
        let sampled_tile = sample(&candidate_map[p], rng);

        // 6. Propagate constraints.
        for direction in enum_iterator::all::<Direction>() {
            let (dx, dy) = direction.step();
            let q = (x + dx, y + dy);

            if !tile_map.inbounds(q) {
                continue;
            }

            // Verify consistency
            if let Some(other) = tile_map[q] {
                if !sampled_tile.connects(&other, direction) {
                    candidate_map[p].remove(&sampled_tile);
                    continue 'outer;
                }
                continue;
            }

            queue.insert(q);

            candidate_map[q] = candidate_map[q]
                .iter()
                .cloned()
                .filter(|a| sampled_tile.connects(a, direction))
                .collect();
        }

        tile_map[p] = Some(sampled_tile);
        queue.remove(&p);
    }

    Some(tile_map)
}

/// Compute the entropy of a given candidates set.
fn entropy(candidates: &HashSet<RotatedTile>) -> usize {
    candidates
        .iter()
        .map(|candidate| candidate.tile().weight())
        .sum()
}

fn sample<R: RngCore>(candidates: &HashSet<RotatedTile>, rng: &mut R) -> RotatedTile {
    let candidates = candidates.iter().collect::<Vec<_>>();
    let weights = candidates
        .iter()
        .map(|candidate| candidate.tile().weight())
        .collect::<Vec<_>>();
    let dist = WeightedIndex::new(weights).unwrap();
    *candidates[rng.sample(dist)]
}
