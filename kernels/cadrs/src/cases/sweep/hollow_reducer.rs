//! A hollow square tube tapering along the y axis from y = 0 to y = 40: square sections centered
//! on the y axis, 24 × 24 with a hole 20 × 20 at y = 0 and 12 × 12 with a hole 8 × 8 at y = 40,
//! every corner running straight from end to end.
//!
//! cadrs's Loft takes a section that is one closed contour, not a region with a hole ("a loft
//! profile must be one closed contour"), so the tube is the loft between the outer squares less
//! the loft between the holes: each a solid Loft between two squares drawn on planes square to
//! the y axis (two sections give ruled faces).
use crate::bench::{cut, p3, plane, rectangle, sketch, v3};
use crate::shared::loft::loft;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Profile};

/// The square of half-side `half` about the y axis on the plane at `y`.
fn square(y: f64, half: f64) -> Profile {
    let across = plane(p3(0.0, y, 0.0), v3(1.0, 0.0, 0.0), v3(0.0, 1.0, 0.0));
    sketch(across, vec![rectangle(-half, -half, half, half)])
}

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let outside = loft(k, vec![square(0.0, 12.0), square(40.0, 6.0)])?;
    let hole = loft(k, vec![square(0.0, 10.0), square(40.0, 4.0)])?;
    cut(k, outside, hole)
}
