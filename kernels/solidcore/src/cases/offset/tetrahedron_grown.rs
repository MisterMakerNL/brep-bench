//! The regular tetrahedron with corners (3, 3, 3), (3, −3, −3), (−3, 3, −3) and (−3, −3, 3) grown
//! by 1: every point within 1 of it, its edges pieces of cylinders and its corners pieces of
//! balls, the kernel's offset with its convex edges rounded. The tetrahedron is the pyramid on the
//! triangle of its first three corners to the fourth.
use crate::bench::polygon;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::sweep::loft_to_point;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let base = polygon(&[[3.0, 3.0, 3.0], [3.0, -3.0, -3.0], [-3.0, 3.0, -3.0]]);
    let tetrahedron = loft_to_point(&base, [-3.0, -3.0, 3.0])?;
    offset_rounded(&tetrahedron, 1.0)
}
