//! The upper half of the ellipsoid x²/36 + y²/16 + z²/9 ≤ 1, made the way brepjs makes an
//! ellipsoid (a ball of radius 1 scaled by 6, 4 and 3 along the axes), with the ball halved first:
//! the ball less a box filling everything below z = 0 and reaching past it on every other side,
//! then scaled. The scale keeps the plane z = 0 where it is, so this is the ellipsoid halved.
//! (Cutting the scaled ellipsoid instead sends brepkit's Boolean to its mesh fallback.)
use brepkit_math::mat::Mat4;
use brepkit_operations::primitives::{make_box, make_sphere};
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::{cut, moved, placed};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let ball = make_sphere(topo, 1.0, 64)?;
    let below = make_box(topo, 4.0, 4.0, 2.0)?;
    let below = moved(topo, below, -2.0, -2.0, -2.0)?;
    let dome = cut(topo, ball, below)?;
    Ok(placed(topo, dome, Mat4::scale(6.0, 4.0, 3.0))?)
}
