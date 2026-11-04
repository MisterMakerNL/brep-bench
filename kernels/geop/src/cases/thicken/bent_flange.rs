//! A bent sheet — a floor from (−10, ·, 0) to (0, ·, 0), a quarter cylinder of radius 2 about the
//! line through (0, ·, 2) along y, and an upright wall from (2, ·, 2) to (2, ·, 12), for y from 0
//! to 8 — thickened by 1 to the outside of the bend: a bracket 1 thick, its bend's radii 2 and 3.
//! The sheet is the profile drawn in the xz plane, extruded 8 along +y as faces; its normals
//! point toward the bend's axis, so it is thickened behind them.
use crate::shared::surfaces::{Side, sheet_extrude, thicken};
use crate::shared::{Drawing, sketch, zx, zx_at};
use crate::{Bench, Refusal};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut profile = Drawing::new();
    let start = profile.point(zx_at(-10.0, 0.0));
    let bend_start = profile.point(zx_at(0.0, 0.0));
    let bend_end = profile.point(zx_at(2.0, 2.0));
    let top = profile.point(zx_at(2.0, 12.0));
    profile.line(start, bend_start);
    // A quarter turn about (0, 2): clockwise in the sketch, whose second axis is −z.
    profile.arc(bend_start, bend_end, -90.0);
    profile.line(bend_end, top);
    sketch(b, "profile", zx(), profile)?;
    sheet_extrude(b, "sheet", "profile", 8.0)?;
    let floor = b.face([-5.0, 4.0, 0.0])?;
    thicken(b, "bracket", &floor, 1.0, Side::Against)
}
