pub mod acceleration;
pub mod angular_velocity;
pub mod linear_velocity;
pub mod orientation;
pub mod position;

use crate::physics::types::R3;

/// Macro to generate all boilerplate implementations for R3 newtype wrappers
#[macro_export]
macro_rules! r3_impl {
    ($name:ident) => {
        impl $name {
            pub fn new(x: f64, y: f64, z: f64) -> Self {
                $name($crate::physics::types::R3::new(x, y, z))
            }
        }

        impl Default for $name {
            fn default() -> Self {
                $name($crate::physics::types::R3::new(0.0, 0.0, 0.0))
            }
        }

        impl $crate::physics::state::HasR3 for $name {
            fn r3(&self) -> &$crate::physics::types::R3 {
                &self.0
            }

            fn from_r3(r3: $crate::physics::types::R3) -> Self {
                $name(r3)
            }
        }

        impl std::ops::Deref for $name {
            type Target = $crate::physics::types::R3;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

pub trait HasR3 {
    fn r3(&self) -> &R3;
    fn from_r3(r3: R3) -> Self;
}

/// Trait for vector operations in 3D space
pub trait Vector: HasR3 {
    /// Returns a unit vector (magnitude = 1) pointing in the exact same direction
    fn unit_vector(&self) -> Self
    where
        Self: Sized,
    {
        let magnitude = self.r3().magnitude();
        let current_r3 = self.r3();
        let unit_r3 = R3 {
            x: current_r3.x / magnitude,
            y: current_r3.y / magnitude,
            z: current_r3.z / magnitude,
        };
        Self::from_r3(unit_r3)
    }
}

impl<T> Vector for T where T: HasR3 {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::state::{linear_velocity::LinearVelocity, position::Position};

    #[test]
    fn test_vector_unit_vector_implementation() {
        // Test with a 3-4-5 triangle (magnitude = 5)
        let velocity = LinearVelocity::new(3.0, 4.0, 0.0);
        let unit_velocity = velocity.unit_vector();

        // Test both direct access and deref access
        assert!((unit_velocity.0.x - 0.6).abs() < f64::EPSILON);
        assert!((unit_velocity.x - 0.6).abs() < f64::EPSILON); // Deref access
        assert!((unit_velocity.0.y - 0.8).abs() < f64::EPSILON);
        assert!((unit_velocity.y - 0.8).abs() < f64::EPSILON); // Deref access
        assert!((unit_velocity.0.z - 0.0).abs() < f64::EPSILON);
        assert!((unit_velocity.z - 0.0).abs() < f64::EPSILON); // Deref access

        // Test with already normalized vector
        let position = Position::new(1.0, 0.0, 0.0);
        let unit_position = position.unit_vector();
        assert!((unit_position.x - 1.0).abs() < f64::EPSILON); // Using deref
        assert!((unit_position.y - 0.0).abs() < f64::EPSILON);
        assert!((unit_position.z - 0.0).abs() < f64::EPSILON);
    }
}
