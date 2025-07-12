use crate::physics::types::{Grams, MetresPerSecondSquared};

pub mod math;
pub mod state;
pub mod types;

pub const AIR_DENSITY: f64 = 2.0;
pub const GRAVITY: MetresPerSecondSquared = 9.81;

pub trait Object {
    fn mass(&self) -> Grams;
}

#[inline]
fn weight(o: impl Object) -> f64 {
    o.mass() * GRAVITY
}
