use crate::{physics::util::vector::Vector3, vector3_newtype};

/// Torque is the rotational analogue to linear force. What force is to translation, torque is to rotation.
/// It is also referred to as the **moment of force** or simply **Moment**, as such is commonly denoted by *M*.
///
/// The key components we need are:
/// - a [Force Vector *F*](crate::physics::force::ForceVector)
/// - a [Position Vector *r*](crate::physics::state::Position). This is a vector about which the torque is being measured to the point the
/// force is being applied.
///
/// Any forces applied at the center of mass will **always** be a *zero vector* - `(0,0,0)`, because our *r* vector goes from the pivot
/// point to the point where the force is applied, which would be the same point if at center-of-mass.
///
/// It is force times distance.
///
/// Net Torque on a body determines the rate of change of the body's angular momentum. Effectively **angular acceleration**.
///
/// ## Unit
/// The SI unit for `Torque` is (**N⋅m**)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Torque(pub Vector3);

vector3_newtype!(Torque);
