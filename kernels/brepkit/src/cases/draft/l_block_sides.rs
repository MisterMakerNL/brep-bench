//! The L-block through (0, 0), (30, 0), (30, 10), (10, 10), (10, 25), (0, 25) in z = 0, extruded
//! up to z = 12, all six sides drafted by 6° with the pull up +z about the neutral plane z = 0,
//! the floor, each leaning in as it rises, the two at the concave edge too.
use brepkit_topology::explorer::solid_faces;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::blend::{drafted_up, prism};
use crate::solids::{faces_where, near, p};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let outline = [
        p(0.0, 0.0, 0.0),
        p(30.0, 0.0, 0.0),
        p(30.0, 10.0, 0.0),
        p(10.0, 10.0, 0.0),
        p(10.0, 25.0, 0.0),
        p(0.0, 25.0, 0.0),
    ];
    let block = prism(topo, &outline, (0.0, 0.0, 12.0))?;
    // every face but the floor and the top, the two whose boundaries lie wholly at z = 0 or 12
    let caps = faces_where(topo, block, |q| near(q.z(), 0.0) || near(q.z(), 12.0))?;
    let sides: Vec<_> = solid_faces(topo, block)?
        .into_iter()
        .filter(|f| !caps.contains(f))
        .collect();
    Ok(drafted_up(topo, block, &sides, 0.0, 6.0)?)
}
