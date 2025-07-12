use super::HasAxis3D;
use crate::physics::math::Quaternion;
use crate::physics::types::Axis3D;

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

impl HasAxis3D for Orientation {
    fn axis3d(&self) -> &Axis3D {
        &self.0.axis
    }

    fn from_axis3d(axis: Axis3D) -> Self {
        // Create a quaternion with the given axis and zero rotation
        Orientation(Quaternion {
            angle: crate::physics::types::Angle(0.0),
            axis,
        })
    }
}
