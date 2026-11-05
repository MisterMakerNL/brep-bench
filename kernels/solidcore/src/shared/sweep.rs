//! What the solidcore implementations of the sweep cases share: the helical pipe, a circle
//! carried along a helix by the kernel's screw sweep, and a section swept along a path with
//! sharp corners, mitered.
//!
//! The kernel's sweep along a path does not miter: along lines meeting at a corner it fails
//! (status 1001). The mitered cases compose each run and its miters themselves, and that is how
//! they are built ([`mitered`]).
use crate::bench::{circle, fuse_all};
use crate::shared::chamfer::cut_off;
use crate::Refusal;
use solidcore_ops::sweep::{extrude, screw_sweep, Curve, DraftCorner};
use solidcore_ops::Body;
use std::f64::consts::TAU;

/// How far the screw sweep's walls may lie from their true screw surfaces. The pipes here come
/// out within 1e-8 of their true volume and area at this bound; the least the sweep takes,
/// 1e-6, makes its walls of so many spans that the sweep runs past the bench's time limit.
pub const SCREW_TOLERANCE: f64 = 1e-4;

/// The circle of `radius` about the point (`helix_radius`, 0, `z`) square to the right-handed
/// helix about the z axis through that point rising `pitch` a turn, carried `turns` turns along
/// the helix by the kernel's screw sweep: the screw motion keeps the circle square to the helix
/// it rides, so it sweeps the pipe about the helix, its ends flat.
pub fn pipe_along_helix(radius: f64, helix_radius: f64, z: f64, pitch: f64, turns: f64) -> Result<Body, Refusal> {
    // the helix's direction at its start, (0, 2πR, p), made a unit vector
    let (along, up) = (TAU * helix_radius, pitch);
    let length = along.hypot(up);
    let tangent = [0.0, along / length, up / length];
    let section = circle([helix_radius, 0.0, z], tangent, [1.0, 0.0, 0.0], radius);
    Ok(screw_sweep(&[section], [0.0; 3], [0.0, 0.0, 1.0], pitch, turns, None, SCREW_TOLERANCE)?)
}

/// The pipe of the `helical-pipe` cases: radius 1, along the helix of radius 10 about the z axis
/// rising 5 a turn from (10, 0, 0), right-handed, three whole turns.
pub fn helical_pipe() -> Result<Body, Refusal> {
    pipe_along_helix(1.0, 10.0, 0.0, 5.0, 3.0)
}

/// How far past a miter's corner each run is first extruded, before the miter's plane cuts it
/// back: more than any section here reaches beyond its corner along the run.
const PAST_THE_MITER: f64 = 20.0;

/// The section swept along the path through `corners` (closed back to its first, if `closed`),
/// mitered at each corner, as the mitered cases compose it: each run is the section, square to
/// the run and centred on it, extruded straight along it, and at each corner the two runs are
/// cut by, and meet in, the plane through the corner square to the sum of their directions.
/// `section(p, d)` is the section at the point `p` of a run along the unit `d`. An open path's
/// ends are flat, square to its first and last runs.
pub fn mitered(
    corners: &[[f64; 3]],
    closed: bool,
    section: impl Fn([f64; 3], [f64; 3]) -> Vec<Curve>,
) -> Result<Body, Refusal> {
    let n = corners.len();
    let runs = if closed { n } else { n - 1 };
    let direction = |i: usize| {
        let (a, b) = (corners[i % n], corners[(i + 1) % n]);
        let d = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
        let length = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
        (d.map(|x| x / length), length)
    };
    let sum = |a: [f64; 3], b: [f64; 3]| [a[0] + b[0], a[1] + b[1], a[2] + b[2]];
    let mut pieces = Vec::new();
    for i in 0..runs {
        let (d, length) = direction(i);
        // the miter at a run's start, and at its end, if the path turns there
        let before = (closed || i > 0).then(|| direction((i + runs - 1) % runs).0);
        let after = (closed || i + 1 < runs).then(|| direction(i + 1).0);
        let back = if before.is_some() { PAST_THE_MITER } else { 0.0 };
        let on = if after.is_some() { PAST_THE_MITER } else { 0.0 };
        let a = corners[i];
        let start = [a[0] - back * d[0], a[1] - back * d[1], a[2] - back * d[2]];
        let mut run = extrude(&section(start, d), d, back + length + on, 0.0, DraftCorner::Curvature, true)?;
        if let Some(e) = before {
            // keep what is in front of the start's miter
            run = cut_off(&run, a, sum(e, d).map(|x| -x))?;
        }
        if let Some(f) = after {
            run = cut_off(&run, corners[(i + 1) % n], sum(d, f))?;
        }
        pieces.push(run);
    }
    fuse_all(pieces)
}
