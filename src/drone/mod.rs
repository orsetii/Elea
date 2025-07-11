use crate::physics::types::Grams;

pub const WEIGHT: Grams = 200;

pub struct Drone {
    pub yaw: f64,
    pub pitch: f64,
    pub roll: f64,
    pub propellers: [Propeller; 4],
}

pub struct Propeller {
    pub rpm: usize,
    pub rotation_direction: RotationDirection,
}

pub enum RotationDirection {
    Clockwise,
    CounterClockwise,
}
