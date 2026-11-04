//! A solid cone on the z axis, its base the disc of radius 5 in the xy plane and its apex at
//! (0, 0, 10), with everything on the side x < 0 of the yz plane cut away. geop has no cone of
//! its own: the triangle (0, 0), (5, 0), (0, 10) in the xz plane is turned about its side on the
//! z axis. The half where x < 0 is cut away by a block over x from −10 to 0 and y from −10 to
//! 10, extruded from z = −1 to z = 11, past the cone.
use crate::shared::booleans::{boolean, extrude_between};
use crate::shared::{Drawing, made, revolve, sketch, sketch_axis, xy, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_model::{BooleanOp, Combine};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut section = Drawing::new();
    let (_, sides) = section.polygon(&[zx_at(0.0, 10.0), zx_at(0.0, 0.0), zx_at(5.0, 0.0)]);
    sketch(b, "section", zx(), section)?;
    revolve(b, "cone", "section", sketch_axis("section", sides[0]), 360.0, Combine::NewBody)?;
    let mut half = Drawing::new();
    half.rectangle([-10.0, -10.0], [0.0, 10.0]);
    sketch(b, "half", xy(), half)?;
    extrude_between(b, "block", "half", -1.0, 11.0, Combine::NewBody)?;
    boolean(b, "cut", &made("revolve", "cone"), &made("extrude", "block"), BooleanOp::Difference)
}
