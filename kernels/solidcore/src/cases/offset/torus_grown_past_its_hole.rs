//! A solid torus about the z axis, its tube of radius 2 about a middle circle of radius 4, grown
//! by 3: every point within 3 of it, which is every point within 5 of the middle circle, an apple
//! closed over the hole, the kernel's offset with the parts of the moved face that cross trimmed.
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::{make_torus, Body};

pub fn build() -> Result<Body, Refusal> {
    let ring = make_torus([0.0, 0.0, 0.0], 4.0, 2.0)?;
    offset_rounded(&ring, 3.0)
}
