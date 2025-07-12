pub type Grams = f64;
pub type MetresPerSecondSquared = f64;

/// Stores an angle in radians
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Angle(pub f64);

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Axis3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Axis3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn magnitude(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z)).sqrt()
    }
}

impl std::fmt::Debug for Axis3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magnitude() {
        let axis = Axis3D::new(6.0, 8.0, 0.0);
        assert_eq!(axis.magnitude(), 10.0);
    }
}
