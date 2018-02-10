use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    /// Create a new `Vec3`.
    pub fn new(x: T, y: T, z: T) -> Vec3<T> {
        Vec3 { x, y, z }
    }
}

impl<T: Add<Output = T>> Add for Vec3<T> {
    type Output = Vec3<T>;

    /// Add two `Vec3`.
    fn add(self, rhs: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Add<Output = T> + Copy> AddAssign for Vec3<T> {
    /// Add `rhs` to `self`.
    fn add_assign(&mut self, rhs: Vec3<T>) {
        *self = Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Add<Output = T> + Mul<Output = T>> Mul<Vec3<T>> for Vec3<T> {
    type Output = T;

    /// Compute dot product between two `Vec3`.
    fn mul(self, rhs: Vec3<T>) -> T {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vec3<T> {
    type Output = Vec3<T>;

    /// Multiply a `Vec3` by a scalar.
    fn mul(self, rhs: T) -> Vec3<T> {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: Mul<Output = T> + Copy> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, rhs: T) {
        *self = Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Vec3<T> {
    type Output = Vec3<T>;

    /// Divide a `Vec3` by a scalar.
    fn div(self, rhs: T) -> Vec3<T> {
        // TODO deal with divide-by-zero/NaN
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl<T: Div<Output = T> + Copy> DivAssign<T> for Vec3<T> {
    fn div_assign(&mut self, rhs: T) {
        *self = Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn equality_i32() {
        let a = Vec3::new(1, 2, 3);
        let b = Vec3::new(1, 2, 3);
        assert_eq!(a, b);
        let c = Vec3::new(3, 2, 1);
        assert_ne!(a, c);
    }

    #[test]
    fn add_i32() {
        let a = Vec3::new(1, 1, 1);
        let b = Vec3::new(1, 1, 1);
        let c = Vec3::new(2, 2, 2);
        assert_eq!(a + b, c);
        assert_ne!(a + b, a);
    }

    #[test]
    fn add_assign_i32() {
        let mut a = Vec3::new(1, 1, 1);
        a += Vec3::new(1, 1, 1);
        let c = Vec3::new(2, 2, 2);
        assert_eq!(a, c);
    }

    #[test]
    fn dot_i32() {
        let a = Vec3::new(1, 1, 1);
        let b = Vec3::new(1, 2, 3);
        assert_eq!(a * b, 6);
    }

    #[test]
    fn mul_scalar_i32() {
        let a = Vec3::new(1, 1, 1);
        let b = Vec3::new(2, 2, 2);
        assert_eq!(a * 2, b);
    }

    #[test]
    fn mul_scalar_assign_i32() {
        let mut a = Vec3::new(1, 1, 1);
        a *= 2;
        let b = Vec3::new(2, 2, 2);
        assert_eq!(a, b);
    }

    #[test]
    fn div_scalar_i32() {
        let a = Vec3::new(1, 1, 1);
        let b = Vec3::new(2, 2, 2);
        assert_eq!(b / 2, a);
    }

    #[test]
    fn div_scalar_assign_i32() {
        let mut a = Vec3::new(2, 2, 2);
        a /= 2;
        let b = Vec3::new(1, 1, 1);
        assert_eq!(a, b);
    }

    #[test]
    fn equality_f64() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(a, b);
        let c = Vec3::new(3.0, 2.0, 1.0);
        assert_ne!(a, c);
    }

    #[test]
    fn add_f64() {
        let a = Vec3::new(1.0, 1.0, 1.0);
        let b = Vec3::new(1.0, 1.0, 1.0);
        let c = Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(a + b, c);
        assert_ne!(a + b, a);
    }

    #[test]
    fn add_assign_f64() {
        let mut a = Vec3::new(1.0, 1.0, 1.0);
        a += Vec3::new(1.0, 1.0, 1.0);
        let c = Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(a, c);
    }

    #[test]
    fn dot_f64() {
        let a = Vec3::new(1.0, 1.0, 1.0);
        let b = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(a * b, 6.0);
    }

    #[test]
    fn mul_scalar_f64() {
        let a = Vec3::new(1.0, 1.0, 1.0);
        let b = Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(a * 2.0, b);
    }

    #[test]
    fn mul_scalar_assign_f64() {
        let mut a = Vec3::new(1.0, 1.0, 1.0);
        a *= 2.0;
        let b = Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(a, b);
    }

    #[test]
    fn div_scalar_f64() {
        let a = Vec3::new(1.0, 1.0, 1.0);
        let b = Vec3::new(2.0, 2.0, 2.0);
        assert_eq!(b / 2.0, a);
    }

    #[test]
    fn div_scalar_assign_f64() {
        let mut a = Vec3::new(2.0, 2.0, 2.0);
        a /= 2.0;
        let b = Vec3::new(1.0, 1.0, 1.0);
        assert_eq!(a, b);
    }
}
