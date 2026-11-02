//! A hub with a stop, and a rib turned round the hub until it meets the stop. The hub: a
//! cylinder of radius 10 on the z axis from z = 0 to 10. The stop: the rectangle 0 ≤ x ≤ 25,
//! 0 ≤ z ≤ 10 of the plane y = 0 revolved 15° about z and turned on to lie between the
//! half-planes at 120° and 135°, joined to the hub. The rib: the rectangle 5 ≤ x ≤ 20, 0 ≤ z ≤ 4
//! of the plane y = 0 revolved about z from +x towards +y, and joined to them.
//!
//! brepkit's revolve turns through a given angle and has no revolution up to a face. So the rib
//! is turned through 120°, the angle of the stop's face it is to meet, where its end lies flat
//! against that face.
use brepkit_math::mat::Mat4;
use brepkit_operations::revolve::revolve;
use brepkit_topology::builder::make_planar_face;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::contact::cylinder_z;
use crate::solids::{fuse, p, placed, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let hub = cylinder_z(topo, 10.0, (0.0, 0.0), 0.0, 10.0)?;
    let section = make_planar_face(
        topo,
        &[
            p(0.0, 0.0, 0.0),
            p(25.0, 0.0, 0.0),
            p(25.0, 0.0, 10.0),
            p(0.0, 0.0, 10.0),
        ],
        1e-7,
    )?;
    let stop = revolve(
        topo,
        section,
        p(0.0, 0.0, 0.0),
        v(0.0, 0.0, 1.0),
        15f64.to_radians(),
    )?;
    let stop = placed(topo, stop, Mat4::rotation_z(120f64.to_radians()))?;
    let base = fuse(topo, hub, stop)?;
    let profile = make_planar_face(
        topo,
        &[
            p(5.0, 0.0, 0.0),
            p(20.0, 0.0, 0.0),
            p(20.0, 0.0, 4.0),
            p(5.0, 0.0, 4.0),
        ],
        1e-7,
    )?;
    let rib = revolve(
        topo,
        profile,
        p(0.0, 0.0, 0.0),
        v(0.0, 0.0, 1.0),
        120f64.to_radians(),
    )?;
    Ok(fuse(topo, base, rib)?)
}
