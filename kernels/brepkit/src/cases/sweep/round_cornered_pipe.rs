//! A round pipe turned round a sharp corner with a round joint. brepkit's sweep has a round
//! corner mode, but it is not implemented (the sweep falls back to its smooth mode, which has no
//! single section at a kink), so the pipe is built as the description composes it: each run is
//! the disc of radius 3 swept straight along it, ending flat at the corner (an extrusion of the
//! disc, from (−30, 0, 0) along +x and from the origin along +y), and the wedge outside the
//! corner is the disc turned a quarter turn about the z axis, a quarter of the ball of radius 3
//! about the origin. The rest of that ball lies inside the two runs, so the pipe is the two runs
//! fused, then fused with the ball. (Fusing the ball to one run first makes the second fuse fall
//! back to brepkit's mesh Boolean.)
use brepkit_operations::extrude::extrude;
use brepkit_operations::primitives::make_sphere;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::sweep::disc;
use crate::solids::{fuse, p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let first = disc(topo, p(-30.0, 0.0, 0.0), v(1.0, 0.0, 0.0), 3.0)?;
    let first = extrude(topo, first, v(1.0, 0.0, 0.0), 30.0)?;
    let second = disc(topo, p(0.0, 0.0, 0.0), v(0.0, 1.0, 0.0), 3.0)?;
    let second = extrude(topo, second, v(0.0, 1.0, 0.0), 30.0)?;
    let joint = make_sphere(topo, 3.0, 64)?;
    let runs = fuse(topo, first, second)?;
    Ok(fuse(topo, runs, joint)?)
}
