pub type Grams = f64;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Angle(pub f64);

impl std::ops::Deref for Angle {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
