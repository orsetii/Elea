use crate::physics::types::R3;
use crate::r3_impl;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Drag(pub R3);

r3_impl!(Drag);
