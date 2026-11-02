//! A block from the origin to (10, 4, 6), the two edges of its top face that run along x (at
//! y = 0 and y = 4) rounded to radius 2 in one fillet, half the top's width, so its top becomes a
//! half-cylinder.
use brepkit_operations::primitives::make_box;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::blend::rounded;
use crate::solids::{edges_where, near};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let block = make_box(topo, 10.0, 4.0, 6.0)?;
    let long_top = edges_where(topo, block, |q| {
        near(q.z(), 6.0) && (near(q.y(), 0.0) || near(q.y(), 4.0))
    })?;
    Ok(rounded(topo, block, &long_top, 2.0)?)
}
