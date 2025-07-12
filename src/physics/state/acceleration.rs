use super::HasAxis3D;
use crate::physics::types::Axis3D;

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
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Acceleration {
    pub coordinates: Axis3D,
}

impl Acceleration {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Acceleration {
            coordinates: Axis3D { x, y, z },
        }
    }
}

impl Default for Acceleration {
    fn default() -> Self {
        Acceleration::new(0.0, 0.0, 0.0)
    }
}

impl HasAxis3D for Acceleration {
    fn axis3d(&self) -> &Axis3D {
        &self.coordinates
    }

    fn from_axis3d(axis: Axis3D) -> Self {
        Acceleration { coordinates: axis }
    }
}
