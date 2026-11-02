//! The box from the origin to (20, 12, 10), its four sides drafted by 5° the other way (a draft
//! of −5°) with the pull up +z about the neutral plane z = 0, the floor, each leaning out as it
//! rises; the top grows to meet them.
use brepkit_operations::primitives::make_box;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::blend::{drafted_up, faces_on};
use crate::solids::near;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let block = make_box(topo, 20.0, 12.0, 10.0)?;
    let sides = faces_on(
        topo,
        block,
        &[
            &|q| near(q.x(), 0.0),
            &|q| near(q.x(), 20.0),
            &|q| near(q.y(), 0.0),
            &|q| near(q.y(), 12.0),
        ],
    )?;
    Ok(drafted_up(topo, block, &sides, 0.0, -5.0)?)
}
