//! A round flange on the z axis: a plate of radius 32 from z = 0 to 6 and a hub of radius 15 from
//! z = 6 to 16, united; a bore of radius 8 cut through both; and six bolt holes of radius 3 on
//! the circle of radius 24, one every 60° from +x, each cut through the plate in turn. The
//! cutting cylinders run past the faces they go through.
//!
//! The holes are cut one after another: brepkit's `compound_cut` of the six at once returns the
//! flange with none of them cut.
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;
use std::f64::consts::PI;

use crate::solids::contact::cylinder_z;
use crate::solids::{cut, fuse};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let plate = cylinder_z(topo, 32.0, (0.0, 0.0), 0.0, 6.0)?;
    let hub = cylinder_z(topo, 15.0, (0.0, 0.0), 6.0, 16.0)?;
    let flange = fuse(topo, plate, hub)?;
    let bore = cylinder_z(topo, 8.0, (0.0, 0.0), -1.0, 17.0)?;
    let mut flange = cut(topo, flange, bore)?;
    for k in 0..6 {
        let a = k as f64 * PI / 3.0;
        let hole = cylinder_z(topo, 3.0, (24.0 * a.cos(), 24.0 * a.sin()), -1.0, 7.0)?;
        flange = cut(topo, flange, hole)?;
    }
    Ok(flange)
}
