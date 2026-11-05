//! A solid torus about the z axis, centered on the origin, its tube of radius 3 about a middle
//! circle of radius 10, shrunk by 1.5: the solid of every point at least 1.5 from its outside,
//! the torus about the same middle circle with a tube of radius 1.5.
//!
//! cadrs has no torus primitive: the torus is the tube's circular section revolved about the
//! z axis. Then cadrs's offset inward.
use crate::bench::{p3, torus, v3};
use crate::shared::offset::shrunk;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let t = torus(k, p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0), 10.0, 3.0)?;
    shrunk(k, t, 1.5)
}
