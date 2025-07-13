use crate::{physics::util::vector::Vector3, vector3_newtype};

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
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Position(pub Vector3);

vector3_newtype!(Position);
