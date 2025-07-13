//! Eculidean Vector types used in Elea, to represent various mechanics.
//!

/// # Overview
/// Denotes the position of an object in 3D space as
/// cartesian coordinates.
///
/// # Cartesian Coordinates
/// When we talk about cartesian coordinates as a
/// euclidean vector we are referring to the displacement
/// vector from the origin (0,0,0) to the point (x,y,z)
///
/// ## Direction
/// Direction is determined by the ratios between x, y and z.
/// (3,4,5) point in the same direction as (6,8,10) they're
/// parallel because one is just a scalar multiple of the other.
///
/// ## Magnitude
/// The magnitude is calculated using the pythagoren theoroem
/// extended to 3d: `sqrt((x*x)+(y*y)+(z*z))`
///
pub type Position = Vector3;

/// # Overview
/// Acceleration is the rate of change of velocity of an object
/// with respect to time.
///
/// What is important to remember, the units for
/// acceleration are `m/s ** 2` or `m/s ** -2`
/// This is because we are doing *velocity* (`m/s`)
/// over *time* (`s`)
///
/// If we have a velocity delta of 3 `m/s` over
/// 2 seconds, we have an average acceleration of `1.5 m/s ** 2`
/// So, on average, for every second that passes the velocity
/// is changing by `1.5 m/s`
pub type Accleration = Vector3;

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
/// In short, this is simply changing the magnitude to 1. We retain all
/// direction information and we are just setting magnitude to a simple value.
pub type LinearVelocity = Vector3;

/// How an orientation of an object changes over time
///
/// It behaves very similar to 'standard' velocity.
///
/// The unit is *rad ⋅ s−1*
/// We are using *Spin angular velocity* in this project
pub type AngularVelocity = Vector3;
pub type Force = Vector3;

/// Stores an angle in radians

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magnitude() {
        let axis = Vector3::new(6.0, 8.0, 0.0);
        assert_eq!(axis.magnitude(), 10.0);
    }
}
