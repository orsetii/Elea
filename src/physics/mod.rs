//! # Overview
//!
//! This is the base of our Drone and all its data, and the simulation loop.
//!
//! Time stepping is how we convert all our rates of change from our physics ODEs
//! and translate that into real data and actual statistics.
//!
//! It is effectively the simulation version of a game's update loop/function.
//!
//! We must have a known and consistent methodology or any values become meaningless.
//!
//! We are using a fixed timestep, 16.67ms so we have 60 timesteps per second.
//! There **is** a performance tradeoff here, more updates is more calculations of course.
//!
//!
//! ## Pitfalls
//! ### Time Debt Accumulation
//! If we are even slightly slower than actual time in the simulation (due to updates taking *n+1, where *n* is the timestep)
//! we will fall behind actual real-world time more and more, the longer it goes on.
//! There are some solutions like frame skipping but that is for a later date. TODO review this!

use crate::physics::types::Grams;

pub mod body;
pub mod force;
pub mod math;
pub mod state;
pub mod types;

pub const AIR_DENSITY: f64 = 2.0;
pub const WEIGHT: Grams = 200.0;
pub const DEFAULT_DELTATIME_MS: u64 = 16;
