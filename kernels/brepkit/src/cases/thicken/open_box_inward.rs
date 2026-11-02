//! A sheet of five faces of the cube from the origin to (10, 10, 10), its floor and four walls
//! with no top, thickened by 1 into the cube, each moved face trimmed where it meets the others.
//!
//! brepkit's thicken takes one face, so the tray is thickened as five faces of the cube by
//! brepkit's shell inward, the cube's top its opening.
use brepkit_operations::primitives::make_box;
use brepkit_topology::explorer::solid_faces;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::offset::thicken_faces;
use crate::solids::{faces_where, near};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let cube = make_box(topo, 10.0, 10.0, 10.0)?;
    let top = faces_where(topo, cube, |q| near(q.z(), 10.0))?;
    let tray: Vec<_> = solid_faces(topo, cube)?
        .into_iter()
        .filter(|f| !top.contains(f))
        .collect();
    Ok(thicken_faces(topo, cube, &tray, -1.0)?)
}
