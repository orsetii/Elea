pub mod body;
pub mod force;
pub mod math;
pub mod state;
pub mod torque;
mod util;

pub const AIR_DENSITY: f64 = 2.0;
pub const WEIGHT: util::types::Kilograms = 200.0;
pub const DEFAULT_DELTATIME_MS: u64 = 16;
