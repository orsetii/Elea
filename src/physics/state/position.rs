use super::HasAxis3D;
use crate::physics::types::Axis3D;

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
/// TODO: To speed this up, since this code WILL be
/// passed through a LOT, we will override the
/// std::ops and make it as fast as possible.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub coordinates: Axis3D,
}

impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Position {
            coordinates: Axis3D { x, y, z },
        }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position::new(0.0, 0.0, 0.0)
    }
}

impl HasAxis3D for Position {
    fn axis3d(&self) -> &Axis3D {
        &self.coordinates
    }

    fn from_axis3d(axis: Axis3D) -> Self {
        Position { coordinates: axis }
    }
}
