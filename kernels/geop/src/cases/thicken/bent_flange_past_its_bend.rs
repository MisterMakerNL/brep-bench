//! The bent sheet of `thicken/bent-flange` — a floor, a quarter cylinder of radius 2 about the
//! line through (0, ·, 2) along y, and an upright wall, for y from 0 to 8 — thickened by 3 to the
//! inside of the bend, more than the bend's radius: the bend's face, moved 3 toward its axis,
//! would turn inside out, so the floor and the wall moved meet instead at a concave edge along
//! (−1, y, 3). The sheet is the profile drawn in the xz plane, extruded 8 along +y as faces; its
//! normals point toward the bend's axis, so it is thickened along them, and geop's thicken
//! drops a face whose offset collapses.
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
    thicken(b, "bracket", &floor, 3.0, Side::Along)
}
