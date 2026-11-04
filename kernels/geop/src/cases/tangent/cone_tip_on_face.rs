//! A block from (−10, −10, 0) to (10, 10, 10) united with a solid cone on the z axis, its apex at
//! the origin (touching the block's bottom face from inside) and its base the disc of radius 5
//! at z = 15. The block is a square on the xy plane extruded 10 up. geop has no cone of its own:
//! the triangle (0, 0), (5, 15), (0, 15) in the xz plane is turned about its side on the z axis.
//! Then the boolean union.
use crate::shared::booleans::boolean;
use crate::shared::{Drawing, extrude, made, revolve, sketch, sketch_axis, xy, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_model::{BooleanOp, Combine};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([-10.0, -10.0], [10.0, 10.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "block", "base", 10.0, Combine::NewBody)?;
    let mut section = Drawing::new();
    let (_, sides) = section.polygon(&[zx_at(0.0, 15.0), zx_at(0.0, 0.0), zx_at(5.0, 15.0)]);
    sketch(b, "section", zx(), section)?;
    revolve(b, "cone", "section", sketch_axis("section", sides[0]), 360.0, Combine::NewBody)?;
    boolean(b, "union", &made("extrude", "block"), &made("revolve", "cone"), BooleanOp::Union)
}
