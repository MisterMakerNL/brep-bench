//! The block from the origin to (40, 30, 15) less the pocket from (8, 8, 5) to (32, 22, 15) (the
//! cutting box runs on past the top), the pocket's four walls drafted by 5° with the pull up +z
//! about the neutral plane z = 5, the pocket's floor, each leaning away from the pocket's middle
//! as it rises.
use brepkit_operations::primitives::make_box;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::blend::{drafted_up, faces_on};
use crate::solids::{cut, moved, near};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let block = make_box(topo, 40.0, 30.0, 15.0)?;
    let pocket = make_box(topo, 24.0, 14.0, 11.0)?;
    let pocket = moved(topo, pocket, 8.0, 8.0, 5.0)?;
    let part = cut(topo, block, pocket)?;
    // the walls: the faces on the pocket's four sides
    let walls = faces_on(
        topo,
        part,
        &[
            &|q| near(q.x(), 8.0),
            &|q| near(q.x(), 32.0),
            &|q| near(q.y(), 8.0),
            &|q| near(q.y(), 22.0),
        ],
    )?;
    Ok(drafted_up(topo, part, &walls, 5.0, 5.0)?)
}
