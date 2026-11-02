//! The box from the origin to (30, 10, 10), its four sides drafted by 30° with the pull up +z
//! about the neutral plane z = 0, the floor, so steeply that the long sides meet below the top and
//! the top face is consumed.
use brepkit_operations::primitives::make_box;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::blend::{drafted_up, faces_on};
use crate::solids::near;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let block = make_box(topo, 30.0, 10.0, 10.0)?;
    let sides = faces_on(
        topo,
        block,
        &[
            &|q| near(q.x(), 0.0),
            &|q| near(q.x(), 30.0),
            &|q| near(q.y(), 0.0),
            &|q| near(q.y(), 10.0),
        ],
    )?;
    Ok(drafted_up(topo, block, &sides, 0.0, 30.0)?)
}
