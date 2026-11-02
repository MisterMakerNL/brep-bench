//! A cube of side 10 at the origin with a saw cut 10⁻⁶ wide from its top down to z = 5: the block
//! between x = 5 and x = 5.000001 is cut away, made to run past the cube's front, back and top.
use brepkit_operations::primitives::make_box;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::contact::block;
use crate::solids::cut;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let cube = make_box(topo, 10.0, 10.0, 10.0)?;
    let saw = block(topo, (5.0, -1.0, 5.0), (5.000001, 11.0, 11.0))?;
    Ok(cut(topo, cube, saw)?)
}
