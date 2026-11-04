//! A sheet with no boundary, the sphere of radius 5 about the origin as a surface alone, thickened
//! by 1 outward: the hollow ball between the spheres of radius 5 and 6. The sheet is the half
//! circle of the xz plane from (0, −5) through (5, 0) to (0, 5) revolved a whole turn about the
//! z axis as a face: closed, it bounds nothing until it is thickened. Its normals point to the
//! centre, so it is thickened behind them.
use crate::shared::surfaces::{Side, sheet_revolve, thicken};
use crate::shared::{Drawing, axis, sketch, zx, zx_at};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut meridian = Drawing::new();
    let (south, north) = (meridian.point(zx_at(0.0, -5.0)), meridian.point(zx_at(0.0, 5.0)));
    // Half a turn about the origin through (5, 0): clockwise in the sketch, whose second axis is
    // −z.
    meridian.arc(south, north, -180.0);
    sketch(b, "meridian", zx(), meridian)?;
    sheet_revolve(b, "sheet", "meridian", axis(FrameAxis::Z), 360.0)?;
    let sphere = b.face([3.0 * 0.6, 3.0 * 0.8, 4.0])?;
    thicken(b, "skin", &sphere, 1.0, Side::Against)
}
