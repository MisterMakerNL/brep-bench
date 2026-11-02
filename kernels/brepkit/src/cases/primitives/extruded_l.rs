//! An L-shaped plate: the outline (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15) made a
//! planar face in the xy plane, and the face extruded 8 along +z.
use brepkit_operations::extrude::extrude;
use brepkit_topology::builder::make_planar_face;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::{p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let outline = [
        p(0.0, 0.0, 0.0),
        p(20.0, 0.0, 0.0),
        p(20.0, 5.0, 0.0),
        p(5.0, 5.0, 0.0),
        p(5.0, 15.0, 0.0),
        p(0.0, 15.0, 0.0),
    ];
    let section = make_planar_face(topo, &outline, 1e-7)?;
    Ok(extrude(topo, section, v(0.0, 0.0, 1.0), 8.0)?)
}
