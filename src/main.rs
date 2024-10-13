use crate::visualize::visualize;
use rand::thread_rng;

mod direction;
mod feature;
mod fixed_map;
mod map;
mod rotated_tile;
mod rotation;
mod tile;
mod visualize;
mod wave_function;

// todo: args
pub struct Args {}

fn main() {
    let args = Args {};

    // todo: seedable rng
    let mut rng = thread_rng();

    let map = wave_function::generate(args, &mut rng).unwrap();

    visualize(map, "output.html")
}
