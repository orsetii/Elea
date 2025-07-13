pub mod orientation;

#[cfg(test)]
mod tests {
    use super::super::util::vector::*;

    #[test]
    fn test_vector_unit_vector_implementation() {
        // Test with a 3-4-5 triangle (magnitude = 5)
        let velocity = LinearVelocity::new(3.0, 4.0, 0.0);
        let unit_velocity = velocity.unit_vector();

        assert!((unit_velocity.x - 0.6).abs() < f64::EPSILON);
        assert!((unit_velocity.y - 0.8).abs() < f64::EPSILON);
        assert!((unit_velocity.z - 0.0).abs() < f64::EPSILON);

        // Test with already normalized vector
        let position = Position::new(1.0, 0.0, 0.0);
        let unit_position = position.unit_vector();
        assert!((unit_position.x - 1.0).abs() < f64::EPSILON);
        assert!((unit_position.y - 0.0).abs() < f64::EPSILON);
        assert!((unit_position.z - 0.0).abs() < f64::EPSILON);
    }
}
