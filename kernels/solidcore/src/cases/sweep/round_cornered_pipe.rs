//! A pipe of radius 3 along the path from (-30, 0, 0) along +x to the origin, then turning a right
//! angle to the left along +y to (0, 30, 0): each run the disc swept straight along it, ending
//! flat at the corner; the wedge left open outside the corner filled by the disc turned about the
//! z axis a quarter turn from square to the first run to square to the second.
use crate::bench::{arc, circle, fuse_all, line};
use crate::Refusal;
use solidcore_ops::sweep::{revolve, sweep_placed, SweepFrame};
use solidcore_ops::Body;
use std::f64::consts::{FRAC_PI_2, PI};

pub fn build() -> Result<Body, Refusal> {
    let corner = [0.0; 3];
    let start = [-30.0, 0.0, 0.0];
    // each run the disc swept straight along it, from its start to the corner and from the
    // corner on
    let (first, _, _) = sweep_placed(
        &[circle(start, [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], 3.0)],
        &[line(start, corner)],
        SweepFrame::Follow,
        true,
    )?;
    let (second, _, _) = sweep_placed(
        &[circle(corner, [0.0, 1.0, 0.0], [0.0, 0.0, 1.0], 3.0)],
        &[line(corner, [0.0, 30.0, 0.0])],
        SweepFrame::Follow,
        true,
    )?;
    // Ops sweeps no path with a corner, round or mitred (`sweep_placed` refuses it), so the
    // joint is made as the description composes it: the disc square to the first run at the
    // corner, turned a quarter turn right-handed about the z axis to square to the second. Its
    // half on the outside of the corner (y <= 0) sweeps the open wedge; the other half turns
    // inside the runs. So the half disc in the plane x = 0 below y = 0, its straight side on
    // the z axis, revolved.
    let half = [
        line([0.0, 0.0, 3.0], [0.0, 0.0, -3.0]),
        // from -z round through -y to +z, about -x
        arc(corner, [-1.0, 0.0, 0.0], [0.0, 0.0, -1.0], 3.0, 0.0, PI),
    ];
    let joint = revolve(&half, corner, [0.0, 0.0, 1.0], FRAC_PI_2, true)?;
    fuse_all([first, second, joint])
}
