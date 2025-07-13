use crate::{physics::util::vector::Vector3, vector3_newtype};

/// How an orientation of an object changes over time
///
/// It behaves very similar to 'standard' velocity.
///
/// We are using *Spin angular velocity* in this project
///
/// ## Unit
/// The SI unit for `AngularVelocity` is (**rad ⋅ s−1**)
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct AngularVelocity(pub Vector3);

vector3_newtype!(AngularVelocity);
