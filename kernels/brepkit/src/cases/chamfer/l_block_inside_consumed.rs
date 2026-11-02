//! The L-block whose section in xz is the polygon through (0, 0), (10, 0), (10, 3), (3, 3),
//! (3, 10), (0, 10), extruded along +y for 10; its one concave edge, the line x = 3, z = 3,
//! chamfered with distance 7 on both faces, adding material until the chamfer consumes both faces
//! beside it.
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::blend::{chamfered, l_block_xz};
use crate::solids::{edges_where, near};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let block = l_block_xz(topo)?;
    let inside = edges_where(topo, block, |q| near(q.x(), 3.0) && near(q.z(), 3.0))?;
    chamfered(topo, block, &inside, 7.0)
}
