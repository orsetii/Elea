use crate::physics::{
    force::{thrust::Thrust, weight::Weight},
    types::R3,
};

mod drag;
mod thrust;
mod weight;

#[derive(Debug)]
pub struct Forces {
    thrust: Thrust,
    weight: Weight,
    drag: drag::Drag,
    // TODO add wind force!
}

impl Forces {
    pub fn net_force(&self) -> R3 {
        // we may need to receive some input about propeller status
        // to determine thrust
        unimplemented!()
    }
}

impl Default for Forces {
    fn default() -> Self {
        Self {
            thrust: Thrust::default(),
            weight: Weight::default(),
            drag: drag::Drag::default(),
        }
    }
}
