use crate::physics::util::vector;

#[derive(Debug)]
pub struct Forces {
    thrust: vector::Force,
    weight: vector::Force,
    drag: vector::Force,
    // TODO add wind force!
}

impl Forces {
    pub fn net_force(&self) -> vector::Force {
        // we may need to receive some input about propeller status
        // to determine thrust
        unimplemented!()
    }
}

impl Default for Forces {
    fn default() -> Self {
        Self {
            thrust: vector::Force::default(),
            weight: vector::Force::default(),
            drag: vector::Force::default(),
        }
    }
}
