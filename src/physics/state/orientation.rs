use crate::physics::math::Quaternion;

//// # Overview
/// This refers to the rotation that is needed to
/// move the object from a reference placement to
/// its current placement.
///
/// We have implemented this using an orientation quaternion
/// (like a normal quaternion but it is rotation relative to a
/// reference coordinate system)
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Orientation(Quaternion);

impl Default for Orientation {
    fn default() -> Self {
        Self(Quaternion::default())
    }
}
