//! A sheet, the upper half of the sphere of radius 5 about the origin, thickened by 5 toward the
//! centre — by its whole radius, so that its moved face shrinks to the origin: the half ball of
//! radius 5 where z ≥ 0. The sheet is the quarter circle of the xz plane from (5, 0) to (0, 5)
//! revolved a whole turn about the z axis as a face, whose normals point to the centre:
//! thickened along them.
use crate::shared::surfaces::{Side, sheet_revolve, thicken};
use crate::shared::{Drawing, axis, sketch, zx, zx_at};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut meridian = Drawing::new();
    let (equator, pole) = (meridian.point(zx_at(5.0, 0.0)), meridian.point(zx_at(0.0, 5.0)));
    // A quarter turn about the origin: clockwise in the sketch, whose second axis is −z.
    meridian.arc(equator, pole, -90.0);
    sketch(b, "meridian", zx(), meridian)?;
    sheet_revolve(b, "sheet", "meridian", axis(FrameAxis::Z), 360.0)?;
    let dome = b.face([3.0 * 0.6, 3.0 * 0.8, 4.0])?;
    thicken(b, "half_ball", &dome, 5.0, Side::Along)
}
