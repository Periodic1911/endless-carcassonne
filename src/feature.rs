/// Features visible on the sides of Carcassonne tiles.
#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub enum Feature {
    City,
    Forest,
    Road,
}
