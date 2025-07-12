use crate::physics::types::R3;
use crate::r3_impl;

/// How an orientation of an object changes over time
///
/// It behaves very similar to 'standard' velocity.
///
/// The unit is *rad ⋅ s−1*
/// We are using *Spin angular velocity* in this project
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AngularVelocity(pub R3);

r3_impl!(AngularVelocity);
