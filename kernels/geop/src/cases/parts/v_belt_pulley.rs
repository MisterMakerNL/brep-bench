//! A pulley turned about the z axis: its section (a hub of radius 12 from z = −14 to 14, a rim of
//! radius 28 from z = −9 to 9, a bore of radius 6) sketched on the xz plane and turned a full
//! turn about the z axis; then the V groove, the triangle (22, 0), (29, −5), (29, 5) reaching
//! past the rim, turned a full turn and cut from it.
use crate::shared::{Drawing, axis, made, revolve, sketch, zx, zx_at};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut section = Drawing::new();
    section.polygon(&[
        zx_at(6.0, -14.0),
        zx_at(12.0, -14.0),
        zx_at(12.0, -9.0),
        zx_at(28.0, -9.0),
        zx_at(28.0, 9.0),
        zx_at(12.0, 9.0),
        zx_at(12.0, 14.0),
        zx_at(6.0, 14.0),
    ]);
    sketch(b, "section", zx(), section)?;
    revolve(b, "pulley", "section", axis(FrameAxis::Z), 360.0, Combine::NewBody)?;

    let mut groove = Drawing::new();
    groove.polygon(&[zx_at(22.0, 0.0), zx_at(29.0, -5.0), zx_at(29.0, 5.0)]);
    sketch(b, "groove", zx(), groove)?;
    revolve(b, "belt", "groove", axis(FrameAxis::Z), 360.0, Combine::cut(made("revolve", "pulley")))
}
