use crate::{physics::util::vector::Vector3, vector3_newtype};

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
///
/// ## Unit
/// The SI unit for `Acceleration` is (**m/s²**)
///
/// This is also expressed sometimes as metre per square second,
/// which **is the same unit** but just calculated slightly differently, with the **same result**.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Acceleration(pub Vector3);

vector3_newtype!(Acceleration);
