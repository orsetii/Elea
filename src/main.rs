use std::time::Duration;

use elea::DroneSimulator;

fn main() -> elea::Result<()> {
    let mut simulator = DroneSimulator::new();

    simulator.start()?;
    loop {
        match simulator.simulation_step() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Simulation error: {}", e);
                return Err(e);
            }
        }
    }
}
