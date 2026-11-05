//! Two balls of radius 5, one about the origin and one about (9.999999, 0, 0), overlapping to a
//! depth of 10⁻⁶: their common part, a lens a millionth thick. Each ball is a half disc revolved
//! (the kernel has no sphere primitive).
use crate::bench::{ball, common, p3};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let a = ball(k, p3(0.0, 0.0, 0.0), 5.0)?;
    let b = ball(k, p3(9.999999, 0.0, 0.0), 5.0)?;
    common(k, a, b)
}
