//! The L-shaped plate (its outline through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15),
//! extruded 8 along +z) grown by 1 with brepkit's offset, whose default joint extends the moved
//! faces until they meet, so every edge stays sharp.
use brepkit_operations::offset_v2::offset_solid_v2;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::offset::l_block;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let plate = l_block(topo)?;
    Ok(offset_solid_v2(topo, plate, 1.0)?)
}
