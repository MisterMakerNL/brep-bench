//! A rod, the cylinder of radius 5 about z from z = 0 to 20, grooved along its length by a wheel
//! of two beads slid 40 along z: tori on one axis along y, the first centred at (11.5, 0, −9) of
//! radii 6 and 2, the second at (11.5, 1.5, −9) of radii 6 and 1.5, united.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism, torus};
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::f64::consts::TAU;

const ROD_RADIUS: f64 = 5.;
const ROD_LENGTH: f64 = 20.;
/// Each bead: its centre, the radius of the circle through its tube's centres, and its tube's.
const BEADS: [(V, f64, f64); 2] = [([11.5, 0., -9.], 6., 2.), ([11.5, 1.5, -9.], 6., 1.5)];
const SLIDE: f64 = 40.;
/// How closely the wheel is meshed for the sweep's membership reads: a hundred-and-fiftieth of
/// the thinner tube's radius.
const BAR: f64 = 0.01;

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let rod = prism(&disc([0., 0., 0.], up, ROD_RADIUS), 0., ROD_LENGTH)?;
    // the wheel: the two beads united, two torus faces meeting in two circles
    let [first, second] =
        BEADS.map(|(centre, major, minor)| torus(centre, [0., 1., 0.], major, minor));
    let wheel = combined(&first?, &second?, Op::Union, 0.)?;
    // the slide along z: a screw that does not turn, advancing its whole length over a roll of τ
    let path = Trajectory::Twist(Screw { origin: BEADS[0].0, axis: up, ratio: 0., advance: SLIDE });
    // the kernel's closed swept solid cut from the rod on one arrangement: each bead's face slid
    // square to its axis, the meridians square to the slide swept into cylinders along z
    let size = rod.size().max(wheel.size());
    cut_from(&rod, &wheel, &path, [0., TAU], BAR, tolerance(size))
        .map_err(|why| format!("the wheel's sweep not built: {why:?}"))
}
