pub type Kilograms = f64;
pub type MetresPerSecondSquared = f64;
pub type Newton = f64;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Angle(pub f64);

impl std::ops::Deref for Angle {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct Dimensions3D {
    height: f64,
    width: f64,
    depth: f64,
}

impl Dimensions3D {
    pub fn new(height: f64, width: f64, depth: f64) -> Self {
        Self {
            height,
            width,
            depth,
        }
    }
}

impl Default for Dimensions3D {
    fn default() -> Self {
        Self {
            height: 0.0,
            width: 0.0,
            depth: 0.0,
        }
    }
}
