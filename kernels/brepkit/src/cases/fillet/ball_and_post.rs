//! A ball of radius 10 about the origin fused with a post, the cylinder of radius 4 on the z axis
//! from z = 0 to 15; the circle where the post leaves the ball, at z = √84, rounded to radius 2,
//! adding material.
use brepkit_operations::primitives::{make_cylinder, make_sphere};
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::blend::rounded;
use crate::solids::{edges_where, fuse};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let ball = make_sphere(topo, 10.0, 64)?;
    let post = make_cylinder(topo, 4.0, 15.0)?;
    let joined = fuse(topo, ball, post)?;
    // the circle of radius 4 on the ball, where the post comes out
    let root = edges_where(topo, joined, |q| {
        (q.z() - 84f64.sqrt()).abs() < 1e-6 && (q.x().hypot(q.y()) - 4.0).abs() < 1e-6
    })?;
    Ok(rounded(topo, joined, &root, 2.0)?)
}
