//! A washer: the rectangle x 6..10, z 0..4 in the xz plane made a planar face and revolved a full
//! turn about the z axis.
use std::f64::consts::TAU;

use brepkit_operations::revolve::revolve;
use brepkit_topology::builder::make_planar_face;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::{p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let rectangle = [p(6.0, 0.0, 0.0), p(10.0, 0.0, 0.0), p(10.0, 0.0, 4.0), p(6.0, 0.0, 4.0)];
    let section = make_planar_face(topo, &rectangle, 1e-7)?;
    Ok(revolve(topo, section, p(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), TAU)?)
}
