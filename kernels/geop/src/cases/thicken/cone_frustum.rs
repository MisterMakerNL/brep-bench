//! A sheet, the side of the cone frustum about the z axis from radius 6 at z = 0 to radius 3 at
//! z = 4, thickened by 1 along its normal away from the axis. The sheet is the slant line of the
//! xz plane from (6, 0) to (3, 4) revolved a whole turn about the z axis as a face, whose
//! normals point toward the axis: thickened behind them. geop's thicken joins each of its rim
//! circles to its moved copy by the ruled face between them, a frustum of a cone.
use crate::shared::surfaces::{Side, sheet_revolve, thicken};
use crate::shared::{Drawing, axis, sketch, zx, zx_at};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut slant = Drawing::new();
    let (bottom, top) = (slant.point(zx_at(6.0, 0.0)), slant.point(zx_at(3.0, 4.0)));
    slant.line(bottom, top);
    sketch(b, "slant", zx(), slant)?;
    sheet_revolve(b, "sheet", "slant", axis(FrameAxis::Z), 360.0)?;
    let side = b.face([4.5 * 0.6, 4.5 * 0.8, 2.0])?;
    thicken(b, "frustum", &side, 1.0, Side::Against)
}
