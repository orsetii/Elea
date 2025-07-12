pub mod acceleration;
pub mod angular_velocity;
pub mod linear_velocity;
pub mod orientation;
pub mod position;

use crate::physics::types::Axis3D;

/// Trait for types that contain or can provide access to Axis3D data
pub trait HasAxis3D {
    fn axis3d(&self) -> &Axis3D;

    /// Create a new instance of this type with the given Axis3D coordinates
    fn from_axis3d(axis: Axis3D) -> Self;
}

/// Trait for vector operations in 3D space
pub trait Vector: HasAxis3D {
    /// Returns a unit vector (magnitude = 1) pointing in the same direction
    fn unit_vector(&self) -> Self
    where
        Self: Sized,
    {
        let magnitude = self.axis3d().magnitude();
        let current_axis = self.axis3d();
        let unit_axis = Axis3D {
            x: current_axis.x / magnitude,
            y: current_axis.y / magnitude,
            z: current_axis.z / magnitude,
        };
        Self::from_axis3d(unit_axis)
    }
}

/// Blanket implementation of Vector for any type that has Axis3D access
impl<T> Vector for T where T: HasAxis3D {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physics::state::{linear_velocity::LinearVelocity, position::Position};

    #[test]
    fn test_vector_unit_vector_implementation() {
        // Test with a 3-4-5 triangle (magnitude = 5)
        let velocity = LinearVelocity::new(3.0, 4.0, 0.0);
        let unit_velocity = velocity.unit_vector();
        assert!((unit_velocity.coordinates.x - 0.6).abs() < f64::EPSILON);
        assert!((unit_velocity.coordinates.y - 0.8).abs() < f64::EPSILON);
        assert!((unit_velocity.coordinates.z - 0.0).abs() < f64::EPSILON);

        // Test with already normalized vector
        let position = Position::new(1.0, 0.0, 0.0);
        let unit_position = position.unit_vector();
        assert!((unit_position.coordinates.x - 1.0).abs() < f64::EPSILON);
        assert!((unit_position.coordinates.y - 0.0).abs() < f64::EPSILON);
        assert!((unit_position.coordinates.z - 0.0).abs() < f64::EPSILON);
    }
}
