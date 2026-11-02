//! A block grooved by a ball of radius 3 carried along a cubic Bézier curve in its top face.
//! brepkit sweeps faces, not solids, so the ball's sweep is built by elementary geometry: a ball
//! moving along a path fills the tube about its centre's path with a ball at each end. The tube
//! is brepkit's sweep of the disc of radius 3, square to the curve where it starts, along the
//! curve (on the case's control points); the balls are about its two ends, (0, 0, 0) and
//! (30, 20, 0). The groove is cut from the box from (−6, −6, −8) to (36, 26, 0).
use brepkit_operations::primitives::{make_box, make_sphere};
use brepkit_operations::sweep::sweep;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::sweep::{bezier, disc};
use crate::solids::{cut, fuse, moved, p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let block = make_box(topo, 42.0, 32.0, 8.0)?;
    let block = moved(topo, block, -6.0, -6.0, -8.0)?;
    let path = bezier(&[
        p(0.0, 0.0, 0.0),
        p(10.0, 0.0, 0.0),
        p(20.0, 20.0, 0.0),
        p(30.0, 20.0, 0.0),
    ])?;
    // the curve leaves (0, 0, 0) along +x
    let section = disc(topo, p(0.0, 0.0, 0.0), v(1.0, 0.0, 0.0), 3.0)?;
    let tube = sweep(topo, section, &path)?;
    let ball = make_sphere(topo, 3.0, 64)?;
    let start = moved(topo, ball, 0.0, 0.0, 0.0)?;
    let ball = make_sphere(topo, 3.0, 64)?;
    let end = moved(topo, ball, 30.0, 20.0, 0.0)?;
    let swept = fuse(topo, tube, start)?;
    let swept = fuse(topo, swept, end)?;
    Ok(cut(topo, block, swept)?)
}
