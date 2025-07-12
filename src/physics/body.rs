use std::time::Duration;

use crate::{
    physics::{
        force::Forces,
        state::{
            acceleration::Acceleration, angular_velocity::AngularVelocity,
            linear_velocity::LinearVelocity, orientation::Orientation, position::Position,
        },
    },
    Result,
};
#[derive(Debug)]
pub struct RigidBody {
    pub position: Position,
    pub orientation: Orientation,
    pub linear_velocity: LinearVelocity,
    pub angular_velocity: AngularVelocity,
    pub acceleration: Acceleration,
    pub forces: Forces,
}

impl RigidBody {
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
}

impl Default for RigidBody {
    fn default() -> Self {
        Self {
            position: Position::default(),
            linear_velocity: LinearVelocity::default(),
            angular_velocity: AngularVelocity::default(),
            acceleration: Acceleration::default(),
            orientation: Orientation::default(),
            forces: Forces::default(),
        }
    }
}
