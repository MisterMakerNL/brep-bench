//! What the solidcore implementations of the swept cases share: the rod the wheels groove, and a
//! torus slid along the z axis.
//!
//! solidcore has no sweep of a solid: its sweeps carry curves (a profile along a line, about an
//! axis, along a path or a screw), never a body. Where the solid a body passes through follows
//! from the description by elementary geometry it is built from those sweeps, as the swept
//! cases' implementations say beside their code; where its boundary is an envelope the kernel
//! would have to compute, the case is refused.
use crate::bench::{arc, line, rotated, translated};
use crate::Refusal;
use solidcore_ops::sweep::{extrude, DraftCorner};
use solidcore_ops::{make_cylinder, make_torus, Body};
use std::f64::consts::PI;

/// The rod the wheels groove: the cylinder of radius 5 about the z axis from z = 0 to z = 20.
pub fn rod() -> Result<Body, Refusal> {
    Ok(make_cylinder([0.0; 3], 5.0, 20.0)?)
}

/// The torus centred on `center`, its axis parallel to the y axis, the circle through its tube's
/// centres of radius `major`, its tube of radius `minor`: the kernel's torus (about the z axis)
/// turned a quarter turn about the x axis.
pub fn torus_along_y(center: [f64; 3], major: f64, minor: f64) -> Result<Body, Refusal> {
    let torus = make_torus([0.0; 3], major, minor)?;
    translated(rotated(torus, [0.0; 3], [1.0, 0.0, 0.0], 90.0)?, center)
}

/// The solid the torus of [`torus_along_y`] fills as it slides without turning up the z axis by
/// `rise`: the torus where it starts and where it ends, and between them what its outline seen
/// along the slide sweeps. Seen along z, a torus whose axis is square to z covers the stadium
/// of its tube's radius about the segment of its middle circle's diameter along x (each circle of
/// the tube shows as an ellipse as tall as the tube, centred on that segment, the two at its
/// ends as circles), so every point it passes through lies in one of the two tori or in that
/// stadium extruded from the start's centre to the end's.
pub fn torus_slid(center: [f64; 3], major: f64, minor: f64, rise: f64) -> Result<[Body; 3], Refusal> {
    let [x, y, z] = center;
    let (left, right) = (x - major, x + major);
    // the stadium in the plane z = `z`, counter-clockwise from above
    let stadium = [
        line([left, y - minor, z], [right, y - minor, z]),
        arc([right, y, z], [0.0, 0.0, 1.0], [0.0, -1.0, 0.0], minor, 0.0, PI),
        line([right, y + minor, z], [left, y + minor, z]),
        arc([left, y, z], [0.0, 0.0, 1.0], [0.0, 1.0, 0.0], minor, 0.0, PI),
    ];
    let slide = extrude(&stadium, [0.0, 0.0, 1.0], rise, 0.0, DraftCorner::Curvature, true)?;
    let start = torus_along_y(center, major, minor)?;
    let end = torus_along_y([x, y, z + rise], major, minor)?;
    Ok([start, slide, end])
}
