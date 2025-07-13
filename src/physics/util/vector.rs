//! Vector base for euclidean vector types used in Elea.

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[macro_export]
macro_rules! vector3_newtype {
    ($name:ident) => {
        impl Default for $name {
            fn default() -> Self {
                Self(Vector3::default())
            }
        }

        impl $name {
            pub fn new(x: f64, y: f64, z: f64) -> Self {
                Self(Vector3::new(x, y, z))
            }
        }

        impl std::ops::Deref for $name {
            type Target = Vector3;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl std::ops::Add<$name> for $name {
            type Output = $name;
            fn add(self, rhs: $name) -> Self::Output {
                $name(self.0 + rhs.0)
            }
        }

        impl std::ops::AddAssign<$name> for $name {
            fn add_assign(&mut self, rhs: $name) {
                self.0 += rhs.0;
            }
        }

        impl std::ops::Sub<$name> for $name {
            type Output = $name;
            fn sub(self, rhs: $name) -> Self::Output {
                $name(self.0 - rhs.0)
            }
        }

        impl std::ops::SubAssign<$name> for $name {
            fn sub_assign(&mut self, rhs: $name) {
                self.0 -= rhs.0;
            }
        }

        impl std::ops::Mul<$name> for $name {
            type Output = $name;
            fn mul(self, rhs: $name) -> Self::Output {
                $name(self.0 * rhs.0)
            }
        }

        impl std::ops::MulAssign<$name> for $name {
            fn mul_assign(&mut self, rhs: $name) {
                self.0 *= rhs.0;
            }
        }

        impl std::ops::Div<$name> for $name {
            type Output = $name;
            fn div(self, rhs: $name) -> Self::Output {
                $name(self.0 / rhs.0)
            }
        }

        impl std::ops::DivAssign<$name> for $name {
            fn div_assign(&mut self, rhs: $name) {
                self.0 /= rhs.0;
            }
        }

        impl $name {
            pub fn scalar_mul(&self, scalar: f64) -> Self {
                $name(self.0.scalar_mul(scalar))
            }

            pub fn scalar_div(&self, scalar: f64) -> Self {
                $name(self.0.scalar_div(scalar))
            }
        }

        impl std::ops::Neg for $name {
            type Output = $name;
            fn neg(self) -> Self::Output {
                $name(-self.0)
            }
        }
    };
}

/// TODO: To speed this up, since this code WILL be
/// passed through a LOT, we will override the
/// std::ops and make it as fast as possible.
#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    #[inline]
    pub fn magnitude(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }

    pub fn unit_vector(&self) -> Self {
        let mag = self.magnitude();
        Vector3 {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    pub fn scalar_mul(&self, scalar: f64) -> Self {
        Vector3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn scalar_div(&self, scalar: f64) -> Self {
        Vector3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl std::fmt::Debug for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl Default for Vector3 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl Add<Vector3> for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub<Vector3> for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign<Vector3> for Vector3 {
    fn sub_assign(&mut self, rhs: Vector3) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl MulAssign<Vector3> for Vector3 {
    fn mul_assign(&mut self, rhs: Vector3) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Div<Vector3> for Vector3 {
    type Output = Vector3;
    fn div(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl DivAssign<Vector3> for Vector3 {
    fn div_assign(&mut self, rhs: Vector3) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Self::Output {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magnitude() {
        let axis = Vector3::new(6.0, 8.0, 0.0);
        assert_eq!(axis.magnitude(), 10.0);
    }
}
