use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    /// Create a new `Vec3`.
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }

    /// Compute magnitude.
    pub fn magnitude(&self) -> f64 {
        Vec3::l2_norm(self)
    }

    /// Compute l<sub>2</sub> norm.
    ///
    /// Note: May cause loss of precision.
    pub fn l2_norm(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    // TODO fix the other norms
    //    /// Compute l<sub>1</sub> norm.
    //    pub fn l1_norm(&self) -> T {
    //        if self.x < self.y {
    //            if self.x < self.z {
    //                self.x
    //            }
    //            self.z
    //        }
    //        if self.y < self.z {
    //           self.y
    //        }
    //        self.z
    //    }
    //
    //    /// Computer l<sub>infinity</sub> norm.
    //    pub fn linf_norm(&self) -> T {
    //        if self.x > self.y {
    //            if self.x > self.z {
    //                self.x
    //            }
    //            self.z
    //        }
    //        if self.y > self.z {
    //            self.y
    //        }
    //        self.z
    //    }
}

impl Add for Vec3 {
    type Output = Vec3;

    /// Add two `Vec3`.
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    /// Add `rhs` to `self`.
    fn add_assign(&mut self, rhs: Vec3) {
        *self = *self + rhs;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    /// Subtract two `Vec3`.
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Vec3 {
    /// Subtract `rhs` from `self`.
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = *self - rhs;
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = f64;

    /// Compute dot product between two `Vec3`.
    fn mul(self, rhs: Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    /// Multiply a `Vec3` by a scalar.
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl MulAssign<f64> for Vec3 {
    /// Multiply a `Vec3` by a scalar.
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    /// Divide a `Vec3` by a scalar.
    fn div(self, rhs: f64) -> Vec3 {
        // TODO deal with divide-by-zero/NaN
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl DivAssign<f64> for Vec3 {
    /// Divide a `Vec3` by a scalar.
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn sub_f64() {
        let a = Vec3::new(1.0, 1.0, 1.0);
        let b = Vec3::new(1.0, 1.0, 1.0);
        let c = Vec3::new(0.0, 0.0, 0.0);
        assert_eq!(a - b, c);
        assert_ne!(a - b, a);
    }

    #[test]
    fn sub_assign_f64() {
        let mut a = Vec3::new(1.0, 1.0, 1.0);
        a -= Vec3::new(1.0, 1.0, 1.0);
        let c = Vec3::new(0.0, 0.0, 0.0);
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

    #[test]
    fn magnitude_f64() {
        let a = Vec3::new(2.0, 2.0, 2.0);
        let val: f64 = 2.0;
        let expected = (val.powi(2) + val.powi(2) + val.powi(2)).sqrt();
        assert_eq!(a.magnitude(), expected);
    }
}
