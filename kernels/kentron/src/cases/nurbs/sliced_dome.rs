//! The block under the bicubic Bézier dome of `nurbs/bezier-dome`, with everything above the
//! inclined plane z = 4 + x/2 cut away. The plane passes exactly through the block's corner
//! (0, 0, 4), where the dome meets the sides x = 0 and y = 0. What is cut away is the half-space
//! above the plane, bounded by a square of the plane reaching well past the block.
use crate::bench::{d, p};
use crate::shared::nurbs::bezier_dome;
use kentron::boolean;
use kentron::geometry::surface::{Plane, Surface};
use kentron::make;
use kentron::primitives::half_space;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = bezier_dome(m)?;
    let plane = Plane::new(p(0.0, 0.0, 4.0), d(-0.5, 0.0, 1.0));
    let face = make::face_on_domain(m, Surface::Plane(plane), -60.0, 60.0, -60.0, 60.0)?;
    // the side of the plane its normal (−1/2, 0, 1) points to
    let above = half_space(m, face, p(0.0, 0.0, 50.0))?;
    boolean::difference(m, block, above)
}
