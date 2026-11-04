//! A four-armed star handle: a flat star outline in the xz plane (four tip circles of radius 4
//! centred 33.1 out on the axes, four fillet circles of radius 3 centred 15.9 out on the
//! diagonals, joined by the lines tangent to a tip and the fillet beside it, passing between
//! them) extruded 12 along y with a 2° draft inward: every wall continues as itself, a plane from
//! each side and a cone from each arc.
use crate::bench::{arc, line};
use crate::Refusal;
use solidcore_ops::sweep::{extrude, Curve, DraftCorner};
use solidcore_ops::Body;
use std::f64::consts::{FRAC_1_SQRT_2, FRAC_PI_2, TAU};

const TIP_DISTANCE: f64 = 33.1;
const TIP_RADIUS: f64 = 4.0;
const FILLET_DISTANCE: f64 = 15.9;
const FILLET_RADIUS: f64 = 3.0;

/// The point (x, z) of the outline's plane, y = 0, turned `k` quarter turns about the origin.
fn at([x, z]: [f64; 2], k: usize) -> [f64; 3] {
    let [x, z] = match k % 4 {
        0 => [x, z],
        1 => [-z, x],
        2 => [-x, -z],
        _ => [z, -x],
    };
    [x, 0.0, z]
}

/// The arc of the circle about (x, z) of `radius` in the outline's plane, its angles from +x
/// toward +z, from `start` to `end`, turned `k` quarter turns: counter-clockwise as the outline
/// runs (about −y), or with `clockwise` from `start` back to `end` (about +y, where the angle
/// θ is −θ).
fn round(centre: [f64; 2], radius: f64, start: f64, end: f64, k: usize, clockwise: bool) -> Curve {
    let turn = k as f64 * FRAC_PI_2;
    let (start, end) = (start + turn, end + turn);
    if clockwise {
        arc(at(centre, k), [0.0, 1.0, 0.0], [1.0, 0.0, 0.0], radius, -start, -end)
    } else {
        arc(at(centre, k), [0.0, -1.0, 0.0], [1.0, 0.0, 0.0], radius, start, end)
    }
}

/// The star's outline, as a sketch of it is drawn: tip arcs, fillet arcs and the sides tangent
/// to both.
fn outline() -> Vec<Curve> {
    let d = FILLET_DISTANCE * FRAC_1_SQRT_2;
    let (tip, fillet) = ([TIP_DISTANCE, 0.0], [d, d]);
    // The first arm's side toward the diagonal, the line n·p = c passing between the circles:
    // the tip circle a radius on its positive side and the fillet circle a radius on its
    // negative, so n·(tip − fillet) is the sum of the radii; of the two such lines, the one
    // touching the tip circle on the diagonal's side of the axis.
    let between = [tip[0] - fillet[0], tip[1] - fillet[1]];
    let turn = between[1].atan2(between[0])
        - ((TIP_RADIUS + FILLET_RADIUS) / between[0].hypot(between[1])).acos();
    let n = [turn.cos(), turn.sin()];
    let on_tip = [tip[0] - n[0] * TIP_RADIUS, tip[1] - n[1] * TIP_RADIUS];
    let on_fillet = [fillet[0] + n[0] * FILLET_RADIUS, fillet[1] + n[1] * FILLET_RADIUS];
    let mirror = |[x, z]: [f64; 2]| [x, -z];
    // the tip's outer arc from −a to a, crossing the axis; the fillet's inner arc, facing the
    // origin, clockwise from f to the next arm's side (its mirror image across the diagonal,
    // π/2 − f); the profile is one loop, each curve running on from the one before
    let a = (on_tip[1] - tip[1]).atan2(on_tip[0] - tip[0]);
    let f = (on_fillet[1] - fillet[1]).atan2(on_fillet[0] - fillet[0]).rem_euclid(TAU);
    let mut curves = Vec::new();
    for k in 0..4 {
        curves.push(line(at(mirror(on_fillet), k), at(mirror(on_tip), k)));
        curves.push(round(tip, TIP_RADIUS, -a, a, k, false));
        curves.push(line(at(on_tip, k), at(on_fillet, k)));
        curves.push(round(fillet, FILLET_RADIUS, f, FRAC_PI_2 - f + TAU, k, true));
    }
    curves
}

pub fn build() -> Result<Body, Refusal> {
    Ok(extrude(&outline(), [0.0, 1.0, 0.0], 12.0, 2f64.to_radians(), DraftCorner::Curvature, true)?)
}
