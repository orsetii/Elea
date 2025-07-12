use std::time::SystemTime;

use crate::drone::{Drone, DEFAULT_DELTATIME_MS};

pub mod drone;
pub mod physics;

mod util;
pub use util::{EleaError, Result};

pub struct DroneSimulator {
    pub drone: Drone,
    delta_time: usize,
}

impl DroneSimulator {
    pub fn new() -> Self {
        Self {
            drone: Drone::default(),
            delta_time: DEFAULT_DELTATIME_MS,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn simulation_step(&mut self) -> Result<()> {
        let start_time = SystemTime::now();

        println!("elapsed time: {:#?}", start_time.elapsed()?);
        std::thread::sleep(start_time.elapsed()?);
        Ok(())
    }
}
