//! A sheet, the cap of the sphere of radius 10 about the origin where z ≥ 6, bounded by the
//! circle of radius 8 at z = 6, thickened by 2 away from the sphere's centre: the solid between
//! the spheres of radius 10 and 12 inside the cone from the origin through that circle. The
//! sheet is the arc of the xz plane from (8, 6) to (0, 10) about the origin revolved a whole turn
//! about the z axis as a face, whose normals point to the centre: thickened behind them. geop's
//! thicken joins its rim to the rim's moved copy by the ruled face between them, that cone.
use crate::shared::surfaces::{Side, sheet_revolve, thicken};
use crate::shared::{Drawing, axis, sketch, zx, zx_at};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut meridian = Drawing::new();
    let (rim, pole) = (meridian.point(zx_at(8.0, 6.0)), meridian.point(zx_at(0.0, 10.0)));
    // About the origin, from the rim to the pole: clockwise in the sketch, whose second axis is
    // −z, by the angle whose cosine is 3/5.
    meridian.arc(rim, pole, -(0.6f64.acos().to_degrees()));
    sketch(b, "meridian", zx(), meridian)?;
    sheet_revolve(b, "sheet", "meridian", axis(FrameAxis::Z), 360.0)?;
    let cap = b.face([3.0, 4.0, 75f64.sqrt()])?;
    thicken(b, "cap", &cap, 2.0, Side::Against)
}
