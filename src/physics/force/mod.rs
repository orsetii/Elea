use crate::physics::util::types::{Kilograms, MetresPerSecondSquared};

use crate::{physics::util::vector::Vector3, vector3_newtype};

/// Force is an influence that can cause an object to change velocity.
///
/// We store all forces using this type, including **net force**.
///
/// ## Unit
/// The SI unit for `Force` is the newton (**N**)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ForceVector(pub Vector3);

vector3_newtype!(ForceVector);

/// Crucially this is **not** earths gravitational *force* it is
/// it's gravitiational acceleration. We cannot know the force till we
/// have the mass. If the mass was 1kg, the force is 9.81N.
const EARTH_GRAVITY_ACCELERATION: MetresPerSecondSquared = 9.81;

#[derive(Debug)]
pub struct Forces {
    // we may need to receive some input about propeller status
    // to determine thrust
    thrust: ForceVector,
    /// An object of mass *m* will experience a gravitational force (weight)
    ///  of *F = mg* where *g* is earth's gravitational constant of about 9.81 m/s².
    weight: ForceVector,
    drag: ForceVector,
    // TODO add wind force!
}

impl Forces {
    pub fn calculate_forces(&mut self, mass: Kilograms) -> crate::Result<ForceVector> {
        // TODO calc other forces
        self.calculate_gravitational_force(mass);

        Ok(self.net_force())
    }

    /// The concept of net force is instead of
    /// applying force *a*, then force *b* to the resulting position, then force *c* to that resulting position, and so on; we simply calculate
    /// the resulting (*net*) force that would have resulted from applying each one in turn. *zizek sniff*
    ///
    /// You can do this on paper via the paralellogram technique, we have force *a* and force *b*, we draw both as a line, then draw lines from
    /// their tip paralell to the other force's line. This creates a paralellogram. The vector from the origin to the far corner of the paralellogram is the resultant force!
    ///
    /// This is called the *tip-to-tail* method, so instead of doing it sequentially we get the total effects of all the forces on the body
    ///
    /// The net force **must** be applied at the right point, with the correct associated torque, to replicate exactly the effects of the
    /// original forces
    pub fn net_force(&self) -> ForceVector {
        self.thrust + self.weight + self.drag
    }

    #[inline]
    fn calculate_gravitational_force(&mut self, mass: Kilograms) {
        // This is calculated F = mg purely in the vertical dimension
        self.weight.y = mass * EARTH_GRAVITY_ACCELERATION;
    }
}

impl Default for Forces {
    fn default() -> Self {
        Self {
            thrust: ForceVector::default(),
            weight: ForceVector::default(),
            drag: ForceVector::default(),
        }
    }
}
