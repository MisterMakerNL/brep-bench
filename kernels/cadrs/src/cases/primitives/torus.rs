//! A solid torus centered on the origin, about the z axis, of radius 10 to the middle of its tube
//! and 2 across the tube: the circle of radius 2 about (10, 0) on the Front plane, revolved a
//! whole turn about z (the kernel has no torus primitive).
use crate::bench::{p3, torus, v3};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    torus(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 10.0, 2.0)
}
