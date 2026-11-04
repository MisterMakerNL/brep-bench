//! A sheet, the inner half of the torus about the z axis whose tube of radius 2 runs about a
//! middle circle of radius 6 — the points of it no farther than 6 from the axis, bounded by the
//! circles of radius 6 at z = ±2 — thickened by 1 away from the middle circle, toward the axis.
//! The sheet is the half circle of the xz plane about (6, 0) from (6, −2) through (4, 0) to
//! (6, 2) revolved a whole turn about the z axis as a face, whose normals point to the middle
//! circle: thickened behind them.
use crate::shared::surfaces::{Side, sheet_revolve, thicken};
use crate::shared::{Drawing, axis, sketch, zx, zx_at};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut meridian = Drawing::new();
    let (bottom, top) = (meridian.point(zx_at(6.0, -2.0)), meridian.point(zx_at(6.0, 2.0)));
    // Half a turn about (6, 0) through (4, 0): clockwise in the sketch, whose second axis is −z.
    meridian.arc(bottom, top, -180.0);
    sketch(b, "meridian", zx(), meridian)?;
    sheet_revolve(b, "sheet", "meridian", axis(FrameAxis::Z), 360.0)?;
    let inner = b.face([3.0, 4.0, 3f64.sqrt()])?;
    thicken(b, "ring", &inner, 1.0, Side::Against)
}
