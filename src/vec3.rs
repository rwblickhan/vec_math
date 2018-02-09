use std::ops::{Add, AddAssign};

#[derive(PartialEq, Debug)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 {x, y, z}
    }
}

// TODO implement copy trait

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Add<Output=T> + Copy> AddAssign for Vec3<T> {
    fn add_assign(&mut self, rhs: Vec3<T>) {
        *self = Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn equality_i32() {
        let a = Vec3::new(1,2,3);
        let b = Vec3::new(1,2,3);
        assert_eq!(a,b);
        let c = Vec3::new(3,2,1);
        assert_ne!(a,c);
    }

    #[test]
    fn add_i32() {
        let a = Vec3::new(1,1,1);
        let b = Vec3::new(1,1,1);
        let c = Vec3::new(2,2,2);
        assert_eq!(a+b, c);
    }

    #[test]
    fn add_assign_i32() {
        let mut a = Vec3::new(1,1,1);
        a += Vec3::new(1,1,1);
        let c = Vec3::new(2,2,2);
        assert_eq!(a,c);
    }
}