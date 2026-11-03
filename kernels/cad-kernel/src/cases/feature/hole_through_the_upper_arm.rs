//! A C-shaped bracket, the polygon through (x, z) = (0, 0), (40, 0), (40, 6), (6, 6), (6, 24),
//! (40, 24), (40, 30), (0, 30) in the plane y = −10 extruded 20 along +y, with a hole of radius 4
//! on the vertical axis through (28, 0) drilled down through the upper arm and no further.
//!
//! cad-kernel has no hole feature that drills through to the next face: a hole is a cylinder cut
//! away. So the drill is a cylinder from above the upper arm's top (z = 31) down into the gap
//! between the arms (z = 15), past the arm's underside at z = 24 and short of the lower arm.
use crate::sketch::polygon;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

const SECTION: [[f64; 2]; 8] =
    [[0., 0.], [40., 0.], [40., 6.], [6., 6.], [6., 24.], [40., 24.], [40., 30.], [0., 30.]];
const DEPTH: f64 = 20.;
const HOLE_RADIUS: f64 = 4.;
const HOLE_AT: f64 = 28.;
const DRILL_FROM: f64 = 31.;
const DRILL_TO: f64 = 15.;

pub fn build() -> Result<Brep, String> {
    let points: Vec<_> = SECTION.iter().map(|&[x, z]| [x, -DEPTH / 2., z]).collect();
    // walked counter-clockwise in (x, z), the polygon faces −y: swept back along its normal
    let bracket = prism(&polygon(&points), -DEPTH, 0.)?;
    let drill = prism(&disc([HOLE_AT, 0., DRILL_TO], [0., 0., 1.], HOLE_RADIUS), 0., DRILL_FROM - DRILL_TO)?;
    Ok(combined(&bracket, &drill, Op::Cut, 0.)?)
}
