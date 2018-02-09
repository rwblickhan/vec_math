use std::ops::{Add, AddAssign};

pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T: Add<Output=T>> Add for Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z
        }
    }
}

// TODO fix this, not guaranteed that T is trivially copyable
//impl<T: Add<Output=T>> AddAssign for Vec3<T> {
//    fn add_assign(&mut self, rhs: Vec3<T>) {
//        *self = Vec3 {
//            x: self.x + rhs.x,
//            y: self.y + rhs.y,
//            z: self.z + rhs.z
//        }
//    }
//}