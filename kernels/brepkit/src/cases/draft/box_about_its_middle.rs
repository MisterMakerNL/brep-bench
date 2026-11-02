//! The box from the origin to (20, 12, 10), its four sides drafted by 5° with the pull up +z
//! about the neutral plane z = 5, halfway up, each turning about its line there and leaning in as
//! it rises; the floor grows and the top shrinks to meet them.
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
    Ok(drafted_up(topo, block, &sides, 5.0, 5.0)?)
}
