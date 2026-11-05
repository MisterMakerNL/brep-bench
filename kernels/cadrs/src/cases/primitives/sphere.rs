//! A solid ball of radius 5 centered on the origin: a half disc beside the z axis revolved a whole
//! turn about it (the kernel has no sphere primitive).
use crate::bench::{ball, p3};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    ball(k, p3(0.0, 0.0, 0.0), 5.0)
}
