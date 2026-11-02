//! A four-armed star handle: a flat star outline in the xz plane, extruded 12 along y with a 2°
//! draft.
//!
//! The outline, in the plane y = 0 (u along x, v along z): four tip circles of radius 4 centred
//! 33.1 out on the axes, and four fillet circles of radius 3 centred 15.9 out on the diagonals.
//! Each arm's sides are the lines tangent to its tip circle and the fillet circle beside it that
//! pass between the two; going round, the outline is a tip's outer arc, a side, a fillet's inner
//! arc (facing the origin), a side, and so on round all four arms.
//!
//! The extrusion: from y = 0 to y = 12, its section at y = h the outline moved h·tan 2° inward,
//! so its sides are planes and cones leaning 2° in. That is kentron's drafted extrusion.
//!
//! kentron draws circles tangent to curves but not lines tangent to two circles, so the sides'
//! points of contact are found here by elementary geometry: the line n·x = c with the tip circle a
//! radius on its positive side and the fillet circle a radius on its negative side has
//! n·(tip − fillet) equal to the radii's sum.
use crate::bench::{p, v};
use kentron::geometry::curve::Circle;
use kentron::make;
use kentron::math::Point;
use kentron::sweep;
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_1_SQRT_2;

const TIP_DISTANCE: f64 = 33.1;
const TIP_RADIUS: f64 = 4.0;
const FILLET_DISTANCE: f64 = 15.9;
const FILLET_RADIUS: f64 = 3.0;
const HEIGHT: f64 = 12.0;
const DRAFT_DEGREES: f64 = 2.0;

/// A point (u, v) of the outline's plane in space: x = u, z = v, at y = 0.
fn xz((u, w): (f64, f64)) -> Point {
    p(u, 0.0, w)
}

/// `q` turned about the origin by `k` quarter turns.
fn quarter((u, w): (f64, f64), k: usize) -> (f64, f64) {
    match k % 4 {
        0 => (u, w),
        1 => (-w, u),
        2 => (-u, -w),
        _ => (w, -u),
    }
}

/// `q` mirrored across the u axis, the first arm's centre line.
fn mirror((u, w): (f64, f64)) -> (f64, f64) {
    (u, -w)
}

fn arc(m: &mut Model, start: (f64, f64), through: (f64, f64), end: (f64, f64)) -> Result<Shape, crate::Refusal> {
    // (The edge on the arc's circle itself, over its range: the drafted extrusion turns circles
    // into cones.)
    let arc = Circle::arc_through(xz(start), xz(through), xz(end)).map_err(|e| format!("an arc of the outline: {e}"))?;
    let (first, last) = arc.range();
    Ok(make::edge_from_curve_range(m, arc.basis().clone(), first, last)?)
}

pub fn build(m: &mut Model) -> Result<Shape, crate::Refusal> {
    // The first arm's side toward the diagonal: its points of contact with the tip circle and with
    // the fillet circle, of the two lines between them the one touching the tip on the diagonal's
    // side of the axis.
    let (tip, fillet) = ((TIP_DISTANCE, 0.0), (FILLET_DISTANCE * FRAC_1_SQRT_2, FILLET_DISTANCE * FRAC_1_SQRT_2));
    let between = (tip.0 - fillet.0, tip.1 - fillet.1);
    let length = between.0.hypot(between.1);
    let turn = between.1.atan2(between.0) - ((TIP_RADIUS + FILLET_RADIUS) / length).acos();
    let n = (turn.cos(), turn.sin());
    let on_tip = (tip.0 - n.0 * TIP_RADIUS, tip.1 - n.1 * TIP_RADIUS);
    let on_fillet = (fillet.0 + n.0 * FILLET_RADIUS, fillet.1 + n.1 * FILLET_RADIUS);
    let tip_middle = (TIP_DISTANCE + TIP_RADIUS, 0.0);
    let inner = (FILLET_DISTANCE - FILLET_RADIUS) * FRAC_1_SQRT_2;
    let fillet_middle = (inner, inner);

    // Counter-clockwise from the fillet before the first arm: out along a side, round the tip's
    // outer arc through the axis, in along the other side, round the fillet's inner arc.
    let mut edges = Vec::with_capacity(16);
    for k in 0..4 {
        let (fillet_before, tip_before) = (quarter(mirror(on_fillet), k), quarter(mirror(on_tip), k));
        let (tip_after, fillet_after) = (quarter(on_tip, k), quarter(on_fillet, k));
        edges.push(make::edge_between(m, xz(fillet_before), xz(tip_before))?);
        edges.push(arc(m, tip_before, quarter(tip_middle, k), tip_after)?);
        edges.push(make::edge_between(m, xz(tip_after), xz(fillet_after))?);
        edges.push(arc(m, fillet_after, quarter(fillet_middle, k), quarter(mirror(on_fillet), k + 1))?);
    }
    let outline = make::wire(m, &edges)?;
    let face = make::planar_face(m, outline)?;
    Ok(sweep::extrude_drafted(m, face, v(0.0, HEIGHT, 0.0), DRAFT_DEGREES.to_radians())?)
}
