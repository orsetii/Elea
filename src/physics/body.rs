use std::time::Duration;

use crate::{
    physics::{
        force::Forces,
        state::{
            acceleration::Acceleration, angular_velocity::AngularVelocity,
            linear_velocity::LinearVelocity, orientation::Orientation, position::Position,
        },
        util::types::Dimensions3D,
    },
    Result,
};

// TODO FIXME: Currently all bodys are defined as a cuboid, this will not work for more complex stuff further down the road
#[derive(Debug)]
pub struct RigidBody {
    pub dimensions: Dimensions3D,
    pub position: Position,
    pub orientation: Orientation,
    pub linear_velocity: LinearVelocity,
    pub angular_velocity: AngularVelocity,
    pub acceleration: Acceleration,
    pub forces: Forces,
}

impl RigidBody {
    pub fn new(height: f64, width: f64, depth: f64) -> RigidBody {
        let mut rb = RigidBody::default();
        rb.dimensions = Dimensions3D::new(height, width, depth);
        rb
    }

    pub fn step(&self, dt: Duration) -> Result<()> {
        // Early layout:
        // 1. Calculate and add all forces (done in `Forces.net_force`)
        let net_force = self.forces.net_force();
        // 2. Compute linear and angular acceleration
        //
        // 3. Update velocities.
        //
        // 4. Use velocities to update position and orientation (avoid gimbal lock)
        Ok(())
    }
    const fn center_of_mass(&self) -> Position {
        // for now this IS the position as we have a constant density
        // rigid body
        self.position
    }
}

impl Default for RigidBody {
    fn default() -> Self {
        Self {
            dimensions: Dimensions3D::default(),
            position: Position::default(),
            linear_velocity: LinearVelocity::default(),
            angular_velocity: AngularVelocity::default(),
            acceleration: Acceleration::default(),
            orientation: Orientation::default(),
            forces: Forces::default(),
        }
    }
}
