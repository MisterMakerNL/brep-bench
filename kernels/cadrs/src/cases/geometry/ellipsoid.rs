//! A solid ellipsoid centered on the origin, its semi-axes 6 along x, 4 along y and 3 along z:
//! every point with x²/36 + y²/16 + z²/9 ≤ 1. No two semi-axes are equal, so it is no surface of
//! revolution; a kernel with no ellipsoid may make it as a ball of radius 1 stretched by 6, 4
//! and 3.
//!
//! cadrs cannot: it has no ellipsoid, and no non-uniform scale to stretch a ball with (its
//! `Kernel::scale` scales uniformly, and a `Transform` is a rigid motion). A revolve makes only
//! spheroids, and its loft through elliptic sections only approximates the ellipsoid.
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(_k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    Err(Refusal::from(
        "cadrs has no ellipsoid, and no non-uniform scale to stretch a ball into one \
         (Kernel::scale is uniform, a Transform is a rigid motion); a revolve makes only \
         spheroids",
    ))
}
