//! What the solidcore implementations of the thicken cases share: a sheet thickened
//! (solidcore-ops's [`thicken`]), edges of the result rounded, and the sheets the cases thicken.
//!
//! [`thicken`] is the kernel's `SmApiShellBrepFull` on a sheet with nothing opened, its moved
//! faces extended until they meet wherever they part (`bDoExtendedOffset` on): its edges stay
//! sharp. The kernel's shell has an option to round them instead, which ops does not pass on, but
//! the kernel itself refuses it for these sheets (`SmApiShellBrepFull` with `bDoExtendedOffset`
//! off rejects the angle and the tray as invalid input, as it rejects the box of
//! `shell/box-open-top-outward`). So a rounded thickening is the sharp one with its outer edges
//! rounded ([`rounded_at`]): a round of the thickness on the edge where two moved faces meet is
//! centred on the sheet's edge they were moved from.
use crate::bench::{arc, edges_through_each, line, reloaded};
use crate::shared::nurbs::{polygon, sheet, Face, SEW};
use crate::Refusal;
use solidcore_ops::stitch::stitch;
use solidcore_ops::sweep::{extrude, revolve, DraftCorner};
use solidcore_ops::{blend, thicken, Blend, Body, Corners};
use std::f64::consts::{FRAC_PI_2, PI, TAU};

/// `sheet` thickened by `thickness` along its normals (negative: against them), or
/// (`symmetric`) by half of it to each side, the moved faces extended to meet where they part.
pub fn thickened(sheet: Body, thickness: f64, symmetric: bool) -> Result<Body, Refusal> {
    thicken(&sheet, thickness, symmetric)?;
    Ok(sheet)
}

/// `body` with the edges through `points` rounded to `radius` by ops's blend, the rounds
/// meeting at a corner filled as the kernel builds them.
pub fn rounded_at(body: &Body, points: &[[f64; 3]], radius: f64) -> Result<Body, Refusal> {
    let (body, order) = reloaded(body)?;
    let edges = edges_through_each(&body, &order, points)?;
    blend(&body, Blend::Round, &edges, &[radius], &[], Corners::Filled)?;
    Ok(body)
}

/// The sheet of the flat polygons `faces` (each its corners in order), sewn by ops along the
/// edges they share.
pub fn flat_sheet(faces: &[&[[f64; 3]]]) -> Result<Body, Refusal> {
    let faces: Vec<Face> = faces.iter().map(|corners| Face::Flat(polygon(corners))).collect();
    let (sewn, closed) = stitch(sheet(&faces)?, SEW)?;
    if closed {
        return Err("the faces sewn close into a solid, not a sheet".into());
    }
    Ok(sewn)
}

/// The angle sheet of the `angle-bracket` cases: the floor, the rectangle in the xy plane from
/// the origin to (10, 8, 0), and the wall, the rectangle in the yz plane from the origin to
/// (0, 8, 10), sewn along the crease they share, from the origin to (0, 8, 0).
pub fn angle_sheet() -> Result<Body, Refusal> {
    let floor = [[0.0, 0.0, 0.0], [10.0, 0.0, 0.0], [10.0, 8.0, 0.0], [0.0, 8.0, 0.0]];
    let wall = [[0.0, 0.0, 0.0], [0.0, 8.0, 0.0], [0.0, 8.0, 10.0], [0.0, 0.0, 10.0]];
    flat_sheet(&[&floor, &wall])
}

/// The tray of the `open-box` cases: five faces of the cube from the origin to (10, 10, 10), its
/// floor and its four walls, sewn along the edges they share, open at the top.
pub fn tray() -> Result<Body, Refusal> {
    let p = |x: f64, y: f64, z: f64| [10.0 * x, 10.0 * y, 10.0 * z];
    let floor = [p(0., 0., 0.), p(0., 1., 0.), p(1., 1., 0.), p(1., 0., 0.)];
    let front = [p(0., 0., 0.), p(1., 0., 0.), p(1., 0., 1.), p(0., 0., 1.)];
    let right = [p(1., 0., 0.), p(1., 1., 0.), p(1., 1., 1.), p(1., 0., 1.)];
    let back = [p(1., 1., 0.), p(0., 1., 0.), p(0., 1., 1.), p(1., 1., 1.)];
    let left = [p(0., 1., 0.), p(0., 0., 0.), p(0., 0., 1.), p(0., 1., 1.)];
    flat_sheet(&[&floor, &front, &right, &back, &left])
}

/// The half of the cylinder of radius 5 about the z axis where y ≥ 0, from z = 0 to z = 10: the
/// half circle from (5, 0, 0) through (0, 5, 0) to (−5, 0, 0) extruded 10 up, uncapped.
pub fn half_pipe() -> Result<Body, Refusal> {
    let rim = arc([0.0; 3], [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], 5.0, 0.0, PI);
    Ok(extrude(&[rim], [0.0, 0.0, 1.0], 10.0, 0.0, DraftCorner::Curvature, false)?)
}

/// The upper half of the sphere of radius 5 about the origin, where z ≥ 0: the quarter circle
/// from (5, 0, 0) to (0, 0, 5) in the xz plane turned a whole turn about the z axis, uncapped.
pub fn hemisphere() -> Result<Body, Refusal> {
    // about −y, angle 0 along +x: counter-clockwise from +x up to +z
    let meridian = arc([0.0; 3], [0.0, -1.0, 0.0], [1.0, 0.0, 0.0], 5.0, 0.0, FRAC_PI_2);
    Ok(revolve(&[meridian], [0.0; 3], [0.0, 0.0, 1.0], TAU, false)?)
}

/// The bent sheet of the `bent-flange` cases: the profile in the xz plane along the line from
/// (−10, 0, 0) to the origin, around the quarter circle of radius 2 about (0, 0, 2) to (2, 0, 2)
/// and up the line to (2, 0, 12), extruded 8 along +y, uncapped. Its faces' normals point to
/// the inside of the bend: above the floor, in front of the wall.
pub fn bent_sheet() -> Result<Body, Refusal> {
    let profile = [
        line([-10.0, 0.0, 0.0], [0.0, 0.0, 0.0]),
        // about −y, angle 0 along −z: from straight below the center round to +x
        arc([0.0, 0.0, 2.0], [0.0, -1.0, 0.0], [0.0, 0.0, -1.0], 2.0, 0.0, FRAC_PI_2),
        line([2.0, 0.0, 2.0], [2.0, 0.0, 12.0]),
    ];
    Ok(extrude(&profile, [0.0, 1.0, 0.0], 8.0, 0.0, DraftCorner::Curvature, false)?)
}
