/** Expandable 2d map.
* 
* Can expand from the origin (0, 0) in both positive
* and negative direction.
*/
pub struct Map2D<T> where T: Default + Clone {
    up: Vec<[Vec<T>;2]>,
    down: Vec<[Vec<T>;2]>,
}

impl<T> Map2D<T> where T: Default + Clone {
    /// Create new map filled with the default entry
    /// xmin is how far you can index in the negative axis, xmax is how far in the positive.
    /// For example: `Map2D::new(10, 10, 0, 5)` allows indexing from -10 to 10 in the x axis
    /// (inclusive) and from (0, 10) in the y axis (also inclusive).
    pub fn new(xmin: usize, xmax: usize, ymin: usize, ymax: usize) -> Self {
        Self {
            up: vec![[vec![T::default(); xmin], vec![T::default(); xmax+1]]; ymax+1],
            down: vec![[vec![T::default(); xmin], vec![T::default(); xmax+1]]; ymin],
        }
    }
    
    /// Get an array at index (x, y)
    /// Panics if the point is out-of-bounds
    pub fn get(&self, x: isize, y: isize) -> &T {
        let y_vec = if y < 0 {
            &self.down
        } else {
            &self.up
        };

        let y: usize = Self::signed_index(y);

        let x_vec = if x < 0 {
            &y_vec[y][0]
        } else {
            &y_vec[y][1]
        };

        let x: usize = Self::signed_index(x);

        &x_vec[x]
    }
    
    pub fn get_mut(&mut self, x: isize, y: isize) -> &mut T {
        let y_vec = if y < 0 {
            &self.down
        } else {
            &self.up
        };

        let y: usize = Self::signed_index(y);

        let mut x_vec = if x < 0 {
            &mut y_vec[y][0]
        } else {
            &mut y_vec[y][1]
        };

        let x: usize = Self::signed_index(x);

        &mut x_vec[x]
    }

    fn signed_index(x: isize) -> usize {
        return if x < 0 {
            -(x + 1)
        } else {
            x
        }.try_into().unwrap();

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let _map: Map2D<u8> = Map2D::new(10, 10, 10, 10);
    }

    #[test]
    fn get() {
        let map: Map2D<u8> = Map2D::new(10, 10, 10, 10);
        assert_eq!(map.get(0,0), &u8::default());
        assert_eq!(map.get(10,0), &u8::default());
        assert_eq!(map.get(-10,0), &u8::default());
        assert_eq!(map.get(0,10), &u8::default());
        assert_eq!(map.get(0,-10), &u8::default());
    }

    #[test]
    #[should_panic]
    fn get_out_of_bounds() {
        let map: Map2D<u8> = Map2D::new(10, 10, 10, 10);
        map.get(11,0);
    }
}
