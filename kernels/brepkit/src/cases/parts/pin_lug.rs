//! A flat lug, the plate from (0, −5, 0) to (80, 5, 40), with a pin hole of radius 8 through its
//! thickness on the line x = 60, z = 20: a cylinder along y, run past both faces, cut from it.
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::contact::{block, cylinder_y};
use crate::solids::cut;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let plate = block(topo, (0.0, -5.0, 0.0), (80.0, 5.0, 40.0))?;
    let pin = cylinder_y(topo, 8.0, (60.0, 20.0), -6.0, 6.0)?;
    Ok(cut(topo, plate, pin)?)
}
