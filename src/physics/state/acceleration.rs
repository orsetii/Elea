use crate::physics::types::R3;
use crate::r3_impl;

/// # Overview
/// Acceleration is the rate of change of velocity of an object
/// with respect to time.
///
/// What is important to remember, the units for
/// acceleration are `m/s ** 2` or `m/s ** -2`
/// This is because we are doing *velocity* (`m/s`)
/// over *time* (`s`)
///
/// If we have a velocity delta of 3 `m/s` over
/// 2 seconds, we have an average acceleration of `1.5 m/s ** 2`
/// So, on average, for every second that passes the velocity
/// is changing by `1.5 m/s`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Acceleration(pub R3);

r3_impl!(Acceleration);
