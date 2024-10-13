use crate::map::{Map2D, Point2D};
use rand::{Rng, RngCore};
use std::ops::{Index, IndexMut};

#[derive(Debug)]
pub struct FixedMap2D<T: Default + Clone> {
    map: Vec<T>,
    width: usize,
    height: usize,
    x_min: isize,
    y_min: isize,
}

impl<T: Default + Clone> FixedMap2D<T> {
    /// Sample a random point inside the map.
    pub fn sample_point<R: RngCore>(&self, rng: &mut R) -> Point2D {
        let x = rng.gen_range(self.x_min..self.x_min + self.width as isize);
        let y = rng.gen_range(self.y_min..self.y_min + self.height as isize);
        (x, y)
    }

    pub fn inbounds(&self, (x, y): Point2D) -> bool {
        (self.x_min..self.x_min + self.width as isize).contains(&x)
            & (self.y_min..self.y_min + self.height as isize).contains(&y)
    }

    pub fn fill(&mut self, fill: T) {
        for x in self.x_min..self.x_min + self.width as isize {
            for y in self.y_min..self.y_min + self.height as isize {
                self[(x, y)] = fill.clone();
            }
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

impl<T: Default + Clone> Default for FixedMap2D<T> {
    fn default() -> Self {
        let width = 50;
        let height = 50;
        let x_min = 0;
        let y_min = 0;

        Self {
            map: vec![T::default(); width * height],
            width,
            height,
            x_min,
            y_min,
        }
    }
}

impl<T: Default + Clone> Index<Point2D> for FixedMap2D<T> {
    type Output = T;

    fn index(&self, (x, y): Point2D) -> &Self::Output {
        debug_assert!(self.inbounds((x, y)));
        let x = (x - self.x_min) as usize;
        let y = (y - self.y_min) as usize;
        &self.map[self.width * y + x]
    }
}

impl<T: Default + Clone> IndexMut<Point2D> for FixedMap2D<T> {
    fn index_mut(&mut self, (x, y): Point2D) -> &mut Self::Output {
        debug_assert!(self.inbounds((x, y)));
        let x = (x - self.x_min) as usize;
        let y = (y - self.y_min) as usize;
        &mut self.map[self.width * y + x]
    }
}

impl<T: Default + Clone> Map2D<T> for FixedMap2D<T> {
    fn new(x_min: isize, y_min: isize, width: usize, height: usize) -> Self {
        Self {
            map: vec![T::default(); width * height],
            width,
            height,
            x_min,
            y_min,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let _map: FixedMap2D<u8> = FixedMap2D::new(-10, -10, 20, 20);
    }

    #[test]
    fn get() {
        let map: FixedMap2D<u8> = FixedMap2D::new(-10, -10, 20, 20);
        assert_eq!(map[(0, 0)], u8::default());
    }

    #[test]
    fn get1() {
        let map: FixedMap2D<u8> = FixedMap2D::new(-10, -10, 20, 20);
        assert_eq!(map[(9, 0)], u8::default());
    }
    #[test]
    fn get2() {
        let map: FixedMap2D<u8> = FixedMap2D::new(-10, -10, 20, 20);
        assert_eq!(map[(-10, 0)], u8::default());
    }
    #[test]
    fn get3() {
        let map: FixedMap2D<u8> = FixedMap2D::new(-10, -10, 20, 20);
        assert_eq!(map[(0, 9)], u8::default());
    }
    #[test]
    fn get4() {
        let map: FixedMap2D<u8> = FixedMap2D::new(-10, -10, 20, 20);
        assert_eq!(map[(0, -10)], u8::default());
    }

    #[test]
    #[should_panic]
    fn get_out_of_bounds() {
        let map: FixedMap2D<u8> = FixedMap2D::new(-10, -10, 20, 20);
        map[(10, 0)];
    }

    #[test]
    fn get_mut() {
        let mut map: FixedMap2D<u8> = FixedMap2D::new(-10, -10, 20, 20);
        for i in -10..10 {
            for j in -10..10 {
                let trial_number = (i + j * 20) as u8;
                map[(i, j)] = trial_number;
                assert_eq!(map[(i, j)], trial_number);
            }
        }
    }
}
