#[derive(Debug, Clone, Copy)]
pub struct Propeller {
    pub rpm: usize,
    pub rotation_direction: RotationDirection,
}

impl Default for Propeller {
    fn default() -> Self {
        Self {
            rpm: 0,
            rotation_direction: RotationDirection::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RotationDirection {
    Clockwise,
    CounterClockwise,
}

impl Default for RotationDirection {
    fn default() -> Self {
        RotationDirection::Clockwise
    }
}
