use std::time::{Duration, SystemTime};

use crate::drone::Drone;

pub mod drone;
pub mod physics;

mod util;
pub use util::{EleaError, Result};
pub struct DroneSimulator {
    pub drone: Drone,

    /// Time stepping is how we convert all our rates of change from our physics ODEs
    /// and translate that into real data and actual statistics.
    ///
    /// It is effectively the simulation version of a game's update loop/function.
    ///
    /// We must have a known and consistent methodology or any values become meaningless.
    ///
    /// There **is** a performance tradeoff here, more updates is more calculations of course.
    ///
    /// ## Pitfalls
    ///
    /// ### Time Debt Accumulation
    ///
    /// If we are even slightly slower than actual time in the simulation (due to updates taking *n+1, where *n* is the timestep)
    /// we will fall behind actual real-world time more and more, the longer it goes on.
    /// There are some solutions like frame skipping but that is for a later date. TODO review this!
    delta_time: Duration,
}

impl DroneSimulator {
    pub fn new() -> Self {
        Self {
            drone: Drone::default(),
            delta_time: Duration::from_millis(physics::DEFAULT_DELTATIME_MS),
        }
    }

    pub fn start(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn simulation_step(&mut self) -> Result<()> {
        let start_time = SystemTime::now();

        self.drone.body.step(self.delta_time)?;

        // We store this now, as having it pass some conditions that later would have failed
        // (time passing from the match to the true/false blocks) could cause some real fucky bugs
        // perhaps we should log when its close, and def track if it does end up overruning at the end ig?
        let time_elapsed = start_time.elapsed()?;

        match self.delta_time < time_elapsed {
            true => {
                eprintln!(
                    "TOO LONG TO CALCULATE SIM STEP!!! ELAPSED={}ns DELTATIME={}ns",
                    time_elapsed.as_nanos(),
                    self.delta_time.as_nanos()
                );
                // TODO error out here, and by erroring correctly we probs dont need to log here,
                //  we can log in the main sim function that calls this
            }
            false => {
                println!("Drone State: {:#x?}", self.drone);
                println!(
                    "Simulation step took {}ns, max delta time is {}ns",
                    time_elapsed.as_nanos(),
                    self.delta_time.as_nanos()
                );
                std::thread::sleep(self.delta_time - time_elapsed);
            }
        }
        Ok(())
    }
}
