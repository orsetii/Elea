use crate::physics::types::{Angle, Axis3D};

/// # Overview
/// A quaternion is used in this project to represent a rotation in 3D space.
/// It **always** represents a rotation around an axis that passes through
/// the current coordinate system's origin (0,0,0).
///
/// # The two components
/// We need two key things.
///
/// ## Axis of Rotation
/// The line in 3D space which you want to rotate around.
/// Given as a unit vector (magnitude).
/// So to rotate around the Z-axis our unit axis would be `(0,0,1)`
///
///
/// ## Angle of Rotation
/// How much we want to rotate around that axis.
/// In these docs given in radians - **θ** (Theta)
///
/// # Calculation
///
/// We have a desired rotation of 90 deg
/// Our axis `(x,y,z)` is `(0,0,1)`
///
/// ## Convert angle to radians
///
/// 90deg = `(pi/2)` radians
///
/// If you want to find the `n` in the `pi/n` representation
/// you can simply perform `180/r` where `r` is your angle.
/// So 90deg would be `pi/2`
///
/// ## Calculate Half Angle
/// Simply divide the angle by two!
/// So `θ/2` or (for 90deg) `(pi/2)/2`
///
/// ## Calculate Sine and Cosine
/// Then, with `halfAngle` as our half angle:
/// perform `sin(halfAngle)` and `cos(halfAngle)`.
/// This is used in the construction below.
///
/// ## Construction
/// Construction is done via a four step formula:
///
/// 1. Where *ω* is the scalar (real part) `ω = cos (halfAngle)`
/// 2. `x = X * sin(halfAngle)`
/// 3. `y = Y * sin(halfAngle)`
/// 4. `z = Z * sin(halfAngle)`
///
/// We then put that together into the unit quaternion `(ω,x,y,z)`
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct Quaternion {
    /// How much we are rotating by, in radians
    pub angle: Angle,
    /// On what axis we are rotating
    pub axis: Axis3D,
}

impl Quaternion {
    pub fn new(degree_angle: f64, x: f64, y: f64, z: f64) -> Self {
        let half_angle = degree_angle.to_radians() / 2.0;
        Self {
            angle: Angle(half_angle.cos()),
            axis: Axis3D {
                x: x * half_angle.sin(),
                y: y * half_angle.sin(),
                z: z * half_angle.sin(),
            },
        }
    }
}
impl Default for Quaternion {
    fn default() -> Self {
        Self {
            angle: Angle(1.0),
            axis: Axis3D {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quaternion_90_degrees_z_axis() {
        let quaternion = Quaternion::new(90.0, 0.0, 0.0, 1.0);

        // Check angle (real part)
        assert!((quaternion.angle.0 - 0.7071).abs() < 0.001);
        // Check z component
        assert!((quaternion.axis.z - 0.7071).abs() < 0.001);
    }
}
