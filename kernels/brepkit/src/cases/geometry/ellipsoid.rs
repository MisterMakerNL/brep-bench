//! An ellipsoid with semi-axes 6, 4 and 3 along x, y and z, centred on the origin: brepkit's
//! ellipsoid, a ball of radius 1 scaled by 6, 4 and 3 along the axes (what brepjs's
//! `makeEllipsoid` does). The segment count only sets how finely the ball is meshed.
use brepkit_math::mat::Mat4;
use brepkit_operations::primitives::make_sphere;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::placed;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let ball = make_sphere(topo, 1.0, 64)?;
    Ok(placed(topo, ball, Mat4::scale(6.0, 4.0, 3.0))?)
}
