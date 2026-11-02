//! A pulley on the z axis: a hub of radius 12 from z = −14 to 14 and a rim of radius 28 from
//! z = −9 to 9, united; a bore of radius 6 cut right through (its cylinder run past both ends);
//! and the belt groove cut from the rim, the triangle (22, 0), (29, −5), (29, 5) of the xz plane
//! revolved a full turn about z, reaching 1 past the rim's face.
use brepkit_operations::revolve::revolve;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;
use std::f64::consts::TAU;

use crate::solids::contact::{cylinder_z, outline_face, Run};
use crate::solids::{cut, fuse, p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let hub = cylinder_z(topo, 12.0, (0.0, 0.0), -14.0, 14.0)?;
    let rim = cylinder_z(topo, 28.0, (0.0, 0.0), -9.0, 9.0)?;
    let pulley = fuse(topo, hub, rim)?;
    let bore = cylinder_z(topo, 6.0, (0.0, 0.0), -15.0, 15.0)?;
    let pulley = cut(topo, pulley, bore)?;
    let vee = outline_face(
        topo,
        &[
            (p(22.0, 0.0, 0.0), Run::Line),
            (p(29.0, 0.0, -5.0), Run::Line),
            (p(29.0, 0.0, 5.0), Run::Line),
        ],
    )?;
    let groove = revolve(topo, vee, p(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), TAU)?;
    Ok(cut(topo, pulley, groove)?)
}
