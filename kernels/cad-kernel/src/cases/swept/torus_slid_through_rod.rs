//! A rod, the cylinder of radius 5 about z from z = 0 to 20, grooved along its length by a torus
//! (axis along y, radii 6 and 2) slid without turning along z from centre (11.5, 0, −9) to
//! (11.5, 0, 29).
use cad_kernel::build::{disc, prism, torus};
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::cut_from;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use std::f64::consts::TAU;

const ROD_RADIUS: f64 = 5.;
const ROD_LENGTH: f64 = 20.;
const CENTRE: V = [11.5, 0., -9.];
const MAJOR: f64 = 6.;
const MINOR: f64 = 2.;
const SLIDE: f64 = 38.;
/// How closely the wheel is meshed for the sweep's membership reads: a two-hundredth of its tube's
/// radius.
const BAR: f64 = 0.01;

pub fn build() -> Result<Brep, String> {
    let up = [0., 0., 1.];
    let rod = prism(&disc([0., 0., 0.], up, ROD_RADIUS), 0., ROD_LENGTH)?;
    let wheel = torus(CENTRE, [0., 1., 0.], MAJOR, MINOR)?;
    // the slide along z: a screw that does not turn, advancing its whole length over a roll of τ
    let path = Trajectory::Twist(Screw { origin: CENTRE, axis: up, ratio: 0., advance: SLIDE });
    // the kernel's closed swept solid cut from the rod on one arrangement: a torus slid square to
    // its axis, the meridians square to the slide swept into cylinders along z
    let size = rod.size().max(wheel.size());
    cut_from(&rod, &wheel, &path, [0., TAU], BAR, tolerance(size))
        .map_err(|why| format!("the wheel's sweep not built: {why:?}"))
}
