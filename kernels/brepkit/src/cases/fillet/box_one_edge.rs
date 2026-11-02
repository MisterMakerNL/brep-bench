//! A cube of side 10 from the origin, its edge where the faces x = 10 and y = 10 meet rounded to
//! radius 2.
use brepkit_operations::primitives::make_box;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::blend::rounded;
use crate::solids::{edges_where, near};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let cube = make_box(topo, 10.0, 10.0, 10.0)?;
    let edge = edges_where(topo, cube, |q| near(q.x(), 10.0) && near(q.y(), 10.0))?;
    Ok(rounded(topo, cube, &edge, 2.0)?)
}
