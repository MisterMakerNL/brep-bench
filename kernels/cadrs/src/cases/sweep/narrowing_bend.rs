//! A hollow square duct narrowing as it bends a quarter turn about the z axis along the arc of
//! radius 30 from (30, 0, 0) to (0, 30, 0): its section's outer half-width runs linearly with the
//! angle from 9 to 6, the hole's from 7 to 4, the wall 2 thick.
//!
//! Refused. cadrs's sweep carries its profile rigidly along the path (its profile control turns
//! it, never scales it), and its loft through sections fits its own surfaces between them, not
//! faces swept by segments whose ends follow the spirals the corners trace here, curves the
//! kernel has no form of.
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(_k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    Err(Refusal::from(
        "cadrs's sweep carries its profile rigidly along the path, with no law to scale it, and \
         it has no spiral curve to guide a loft along: it cannot build a section that narrows \
         linearly with the angle turned",
    ))
}
