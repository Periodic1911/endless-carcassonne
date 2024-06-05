use std::ops::{Index, IndexMut};

/// Point in 2-dimensional grid.
pub type Point2D = (isize, isize);

/// Trait for maps that are indexable by 2D points.
pub trait Map2D<T>
where
    T: Default,
    Self: Index<Point2D, Output = T> + IndexMut<Point2D, Output = T>,
{
    fn new(x_min: isize, y_min: isize, width: usize, height: usize) -> Self;
}
