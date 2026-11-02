//! A cylinder of radius 5 on the z axis from z = 0 to 10, grown by 1, its rims rounded.
//!
//! brepkit's rounded offset (`offset_solid_arc_v2`) does not build its arc joints yet, so the
//! cylinder is offset with sharp joints (radius 6, from z = −1 to 11) and the grown cylinder's two
//! rims are then rounded to radius 1: each round lies 1 from the moved side and end, on the tube
//! of radius 1 about the cylinder's own rim.
use brepkit_operations::blend_ops::{all_blended, fillet_v2};
use brepkit_operations::offset_v2::offset_solid_v2;
use brepkit_operations::primitives::make_cylinder;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::{edges_where, near};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let cylinder = make_cylinder(topo, 5.0, 10.0)?;
    let grown = offset_solid_v2(topo, cylinder, 1.0)?;
    let rims = edges_where(topo, grown, |q| near(q.z(), -1.0))?
        .into_iter()
        .chain(edges_where(topo, grown, |q| near(q.z(), 11.0))?)
        .collect::<Vec<_>>();
    Ok(all_blended(fillet_v2(topo, grown, &rims, 1.0)?)?)
}
