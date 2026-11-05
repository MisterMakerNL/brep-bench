//! The ellipsoid x²/36 + y²/16 + z²/9 ≤ 1 (semi-axes 6 along x, 4 along y and 3 along z) with
//! everything below the plane z = 0 cut away: a dome on the flat ellipse x²/36 + y²/16 ≤ 1,
//! rising to 3.
//!
//! cadrs cannot make the ellipsoid (see `geometry/ellipsoid`): it has no ellipsoid, and no
//! non-uniform scale to stretch a ball with. Its half is refused for the same reason.
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(_k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    Err(Refusal::from(
        "cadrs has no ellipsoid, and no non-uniform scale to stretch a ball into one \
         (Kernel::scale is uniform, a Transform is a rigid motion); a revolve makes only \
         spheroids",
    ))
}
