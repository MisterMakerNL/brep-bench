//! A solid torus about the z axis, its tube of radius 3 about a middle circle of radius 10, shrunk
//! by 1.5: the torus whose tube has radius 1.5 about the same circle, the kernel's offset inward.
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::{make_torus, Body};

pub fn build() -> Result<Body, Refusal> {
    let ring = make_torus([0.0, 0.0, 0.0], 10.0, 3.0)?;
    offset_rounded(&ring, -1.5)
}
