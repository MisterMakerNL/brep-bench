//! The box from the origin to (20, 12, 10), its side x = 20 drafted by 10° with the pull up +z
//! about the neutral plane z = 0, leaning in toward −x as it rises.
use brepkit_operations::primitives::make_box;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::blend::drafted_up;
use crate::solids::{faces_where, near};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let block = make_box(topo, 20.0, 12.0, 10.0)?;
    let side = faces_where(topo, block, |q| near(q.x(), 20.0))?;
    Ok(drafted_up(topo, block, &side, 0.0, 10.0)?)
}
