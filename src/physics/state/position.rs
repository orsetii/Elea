use crate::physics::types::R3;
use crate::r3_impl;

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
pub struct Position(pub R3);

r3_impl!(Position);
