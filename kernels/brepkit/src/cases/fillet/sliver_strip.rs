//! A plate from the origin to (20, 10, 2), the two edges of its face y = 10 that run along x (at
//! z = 0 and z = 2) rounded to radius 0.9999 in one fillet, leaving a strip of that face 0.0002
//! tall between the rounds.
use brepkit_operations::primitives::make_box;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::blend::rounded;
use crate::solids::{edges_where, near};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let plate = make_box(topo, 20.0, 10.0, 2.0)?;
    let edges = edges_where(topo, plate, |q| {
        near(q.y(), 10.0) && (near(q.z(), 0.0) || near(q.z(), 2.0))
    })?;
    Ok(rounded(topo, plate, &edges, 0.9999)?)
}
