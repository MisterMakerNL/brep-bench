//! A solid torus about the z axis, centred on the origin, its tube's middle circle of radius 5
//! and the tube of radius 3, cut by the bitangent plane 3x − 4z = 0, keeping what lies on and
//! below it (z ≤ 3x/4). geop has no torus of its own: the circle of radius 3 about (5, 0) in the
//! yz plane is turned about the z axis, so that its seam is in the plane x = 0 on the side
//! y > 0. What lies above the plane is cut away by a block drawn on the xz plane, the
//! quadrilateral above the line z = 3x/4 from x = −10 to 10, extruded 10 each way along y,
//! reaching past the torus.
use crate::shared::booleans::boolean;
use crate::shared::{Drawing, axis, extrude_symmetric, made, revolve, sketch, yz, zx, zx_at};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_model::{BooleanOp, Combine};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut tube = Drawing::new();
    tube.circle([5.0, 0.0], 3.0);
    sketch(b, "tube", yz(), tube)?;
    revolve(b, "ring", "tube", axis(FrameAxis::Z), 360.0, Combine::NewBody)?;
    let mut above = Drawing::new();
    above.polygon(&[zx_at(-10.0, -7.5), zx_at(10.0, 7.5), zx_at(10.0, 10.0), zx_at(-10.0, 10.0)]);
    sketch(b, "above", zx(), above)?;
    extrude_symmetric(b, "block", "above", 20.0, Combine::NewBody)?;
    boolean(b, "cut", &made("revolve", "ring"), &made("extrude", "block"), BooleanOp::Difference)
}
