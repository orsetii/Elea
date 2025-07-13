use crate::{physics::util::vector::Vector3, vector3_newtype};

/// # Overview
/// A measurement of speed in a certain direction
/// of motion.
///
/// The way my brain can best interpret this is;
/// Velocity is simply a force, and we are moving
/// `n` m/s in 3 different directions `x`, `y` and `z`
/// Calculate them up and we get our velocity total, see below!
///
///
/// # Magnitude
/// With a given velocity vector `v`: `(6,8,0) m/s`
///
/// We calculate the magnitude the the 3D pythagorean theorem:
/// `sqrt((6*6) + (8*8) + (0*0)) = sqrt(36 + 64) = 10 m/s`
///
/// # Direction
/// With a given velocity vector `v`: `(6,8,0) m/s`
///
/// We can divide each component by the magnitude to get
/// the direction without any magnitude information:
/// `(6,8,0) / 10 = (0.6,0.8,0)`
///
/// In short, this is simply changing the magnitude to 1. We retain all
/// direction information and we are just setting magnitude to a simple value.
///
/// ## Unit
/// The SI unit for `LinearVelocity` is (**m/s**)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct LinearVelocity(pub Vector3);

vector3_newtype!(LinearVelocity);
