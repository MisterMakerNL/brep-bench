//! A stepped shaft on the z axis made one solid: a cylinder of radius 5 from z = 0 to z = 10
//! united with a cylinder of radius 3 from z = 10 to z = 20 standing on it.
use crate::bench::{cylinder, p3, union, V3};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let thick = cylinder(k, p3(0.0, 0.0, 0.0), V3::z(), 5.0, 10.0)?;
    let thin = cylinder(k, p3(0.0, 0.0, 10.0), V3::z(), 3.0, 10.0)?;
    union(k, thick, thin)
}
