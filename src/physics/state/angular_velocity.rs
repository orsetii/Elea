use super::HasAxis3D;
use crate::physics::types::Axis3D;

/// How an orientation of an object changes over time
///
/// It behaves very similar to 'standard' velocity.
///
/// The unit is *rad ⋅ s−1*
/// We are using *Spin angular velocity* in this project
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AngularVelocity {
    pub coordinates: Axis3D,
}

impl AngularVelocity {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        AngularVelocity {
            coordinates: Axis3D { x, y, z },
        }
    }
}

impl Default for AngularVelocity {
    fn default() -> Self {
        AngularVelocity::new(0.0, 0.0, 0.0)
    }
}

impl HasAxis3D for AngularVelocity {
    fn axis3d(&self) -> &Axis3D {
        &self.coordinates
    }

    fn from_axis3d(axis: Axis3D) -> Self {
        AngularVelocity { coordinates: axis }
    }
}
