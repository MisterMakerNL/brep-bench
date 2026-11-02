//! A block grooved by a ball of radius 3 swung a quarter turn about the z axis. brepkit sweeps
//! faces, not solids, so the ball's sweep is built by elementary geometry: a ball moving along a
//! path fills the tube about its centre's path with a ball at each end. Here the tube is the disc
//! of radius 3 about (12, 0, 0) in the xz plane revolved a quarter turn about the z axis, and the
//! balls are about (12, 0, 0) and (0, 12, 0). The groove is cut from the box from (−6, −6, −8) to
//! (18, 18, 0).
use std::f64::consts::FRAC_PI_2;

use brepkit_operations::primitives::{make_box, make_sphere};
use brepkit_operations::revolve::revolve;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::sweep::disc;
use crate::solids::{cut, fuse, moved, p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let block = make_box(topo, 24.0, 24.0, 8.0)?;
    let block = moved(topo, block, -6.0, -6.0, -8.0)?;
    let section = disc(topo, p(12.0, 0.0, 0.0), v(0.0, 1.0, 0.0), 3.0)?;
    let tube = revolve(topo, section, p(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), FRAC_PI_2)?;
    let ball = make_sphere(topo, 3.0, 64)?;
    let start = moved(topo, ball, 12.0, 0.0, 0.0)?;
    let ball = make_sphere(topo, 3.0, 64)?;
    let end = moved(topo, ball, 0.0, 12.0, 0.0)?;
    let swept = fuse(topo, tube, start)?;
    let swept = fuse(topo, swept, end)?;
    Ok(cut(topo, block, swept)?)
}
