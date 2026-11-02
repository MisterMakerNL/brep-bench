//! A quarter of a washer: the rectangle x 6..10, z 0..4 in the xz plane made a planar face and
//! revolved 90° about the z axis, right-handed (from +x towards +y).
use std::f64::consts::FRAC_PI_2;

use brepkit_operations::revolve::revolve;
use brepkit_topology::builder::make_planar_face;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::{p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let rectangle = [p(6.0, 0.0, 0.0), p(10.0, 0.0, 0.0), p(10.0, 0.0, 4.0), p(6.0, 0.0, 4.0)];
    let section = make_planar_face(topo, &rectangle, 1e-7)?;
    // A positive angle about +z turns right-handed, from +x towards +y.
    Ok(revolve(topo, section, p(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), FRAC_PI_2)?)
}
