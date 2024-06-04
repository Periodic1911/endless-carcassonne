/** 2D map trait
* Anything that can be indexed by a 2D point can implement this trait
*/
pub trait Map2D<T> where T: Default {
    fn new(xmin: isize, ymin: isize, width: usize, height: usize) -> Self;
    fn get(&self, x: isize, y: isize) -> &T;
    fn get_mut(&mut self, x: isize, y: isize) -> &mut T;
}

struct FixedMap<T> where T: Default + Clone {
    vec: Vec<T>,
    width: usize,
    height: usize,
    xmin: isize,
    ymin: isize,
}

impl<T> Map2D<T> for FixedMap<T> where T: Default + Clone {
    fn new(xmin: isize, ymin: isize, width: usize, height: usize) -> Self {
        Self {
            vec: vec![T::default(); width*height],
            width,
            height,
            xmin,
            ymin
        }
    }

    fn get(&self, x: isize, y: isize) -> &T {
        let x = (x-self.xmin) as usize;
        let y = (y-self.ymin) as usize;
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        &self.vec[ x + self.width * y ]
    }

    fn get_mut(&mut self, x: isize, y: isize) -> &mut T {
        let x = (x-self.xmin) as usize;
        let y = (y-self.ymin) as usize;
        debug_assert!(x < self.width);
        debug_assert!(y < self.height);
        &mut self.vec[ x + self.width * y ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let _map: FixedMap<u8> = FixedMap::new(-10, -10, 20, 20);
    }

    #[test]
    fn get() {
        let map: FixedMap<u8> = FixedMap::new(-10, -10, 20, 20);
        assert_eq!(map.get(0,0), &u8::default());
    }

    #[test]
    fn get1() {
        let map: FixedMap<u8> = FixedMap::new(-10, -10, 20, 20);
        assert_eq!(map.get(9,0), &u8::default());
    }
    #[test]
    fn get2() {
        let map: FixedMap<u8> = FixedMap::new(-10, -10, 20, 20);
        assert_eq!(map.get(-10,0), &u8::default());
    }
    #[test]
    fn get3() {
        let map: FixedMap<u8> = FixedMap::new(-10, -10, 20, 20);
        assert_eq!(map.get(0,9), &u8::default());
    }
    #[test]
    fn get4() {
        let map: FixedMap<u8> = FixedMap::new(-10, -10, 20, 20);
        assert_eq!(map.get(0,-10), &u8::default());
    }

    #[test]
    #[should_panic]
    fn get_out_of_bounds() {
        let map: FixedMap<u8> = FixedMap::new(-10, -10, 20, 20);
        map.get(10,0);
    }

    #[test]
    fn get_mut() {
        let mut map: FixedMap<u8> = FixedMap::new(-10, -10, 20, 20);
        for i in -10..10 {
            for j in -10..10 {
                let trial_number = (i+j*20) as u8;
                *map.get_mut(i,j) = trial_number;
                assert_eq!(map.get(i,j), &trial_number);
            }
        }
    }
}
