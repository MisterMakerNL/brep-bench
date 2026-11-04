//! What the solidcore implementations of the draft cases share.
use crate::bench::{edges_through_each, faces_through_each, order, reloaded};
use crate::Refusal;
use solidcore_ops::{blend, draft, make_box, Blend, Body, Corners};
use std::f64::consts::FRAC_PI_4;

/// The faces of `body` through `points` drafted by `degrees` about the neutral plane z = `z`,
/// leaning in as they go along `pull` (a negative angle leans them out).
pub fn draft_about_z(
    body: &Body,
    points: &[[f64; 3]],
    z: f64,
    pull: [f64; 3],
    degrees: f64,
) -> Result<(), Refusal> {
    let faces = faces_through_each(body, points)?;
    draft(body, &order(body)?, &faces, [0.0, 0.0, z], [0.0, 0.0, 1.0], pull, degrees.to_radians())?;
    Ok(())
}

/// The box from `origin` to `origin + size` with its four vertical edges rounded to `radius`.
pub fn rounded_box(origin: [f64; 3], size: [f64; 3], radius: f64) -> Result<Body, Refusal> {
    let (block, order) = reloaded(&make_box(origin, size)?)?;
    let [x0, y0, z0] = origin;
    let (x1, y1, z) = (x0 + size[0], y0 + size[1], z0 + size[2] / 2.0);
    let verticals =
        edges_through_each(&block, &order, &[[x0, y0, z], [x1, y0, z], [x1, y1, z], [x0, y1, z]])?;
    blend(&block, Blend::Round, &verticals, &[radius], &[], Corners::Filled)?;
    Ok(block)
}

/// A point at mid-height `z` on each of the eight sides of a box from (x0, y0) to (x1, y1) whose
/// vertical edges are rounded to `r`: the middle of each plane, and of each quarter cylinder.
pub fn rounded_sides(x0: f64, y0: f64, x1: f64, y1: f64, r: f64, z: f64) -> [[f64; 3]; 8] {
    let (xm, ym) = ((x0 + x1) / 2.0, (y0 + y1) / 2.0);
    let d = r - r * FRAC_PI_4.cos();
    [
        [xm, y0, z],
        [x1 - d, y0 + d, z],
        [x1, ym, z],
        [x1 - d, y1 - d, z],
        [xm, y1, z],
        [x0 + d, y1 - d, z],
        [x0, ym, z],
        [x0 + d, y0 + d, z],
    ]
}

/// The four sides of the box from the origin to `size`, by a point in the middle of each.
pub fn box_sides(size: [f64; 3]) -> [[f64; 3]; 4] {
    let [x, y, z] = size.map(|s| s / 2.0);
    [[x, 0.0, z], [size[0], y, z], [x, size[1], z], [0.0, y, z]]
}
