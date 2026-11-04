//! A solid cone on the z axis, its base the disc of radius 5 in the xy plane and its apex at
//! (0, 0, 10), cut by the plane 4x + z = 10 through the apex, the side 4x + z > 10 cut away. geop
//! has no cone of its own: the triangle (0, 0), (5, 0), (0, 10) in the xz plane is turned about
//! its side on the z axis. The far side is cut away by a block drawn on the xz plane, the
//! quadrilateral beyond the line 4x + z = 10 from (3, −2) to (−0.5, 12), extruded 6 each way
//! along y, past the cone.
use crate::shared::booleans::boolean;
use crate::shared::{Drawing, extrude_symmetric, made, revolve, sketch, sketch_axis, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_model::{BooleanOp, Combine};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut section = Drawing::new();
    let (_, sides) = section.polygon(&[zx_at(0.0, 10.0), zx_at(0.0, 0.0), zx_at(5.0, 0.0)]);
    sketch(b, "section", zx(), section)?;
    revolve(b, "cone", "section", sketch_axis("section", sides[0]), 360.0, Combine::NewBody)?;
    let mut beyond = Drawing::new();
    beyond.polygon(&[zx_at(3.0, -2.0), zx_at(8.0, -2.0), zx_at(8.0, 12.0), zx_at(-0.5, 12.0)]);
    sketch(b, "beyond", zx(), beyond)?;
    extrude_symmetric(b, "block", "beyond", 12.0, Combine::NewBody)?;
    boolean(b, "cut", &made("revolve", "cone"), &made("extrude", "block"), BooleanOp::Difference)
}
