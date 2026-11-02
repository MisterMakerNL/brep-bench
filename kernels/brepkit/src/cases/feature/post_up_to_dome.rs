//! A closed hollow dome with a post inside it. The dome: a plate, the disc of radius 20 from
//! z = −2 to 0, united with the upper half of a ball of radius 20 (split off at z = 0), less the
//! upper half of a ball of radius 18, which leaves a closed half-ball cavity on the plate. The
//! post: the disc of radius 3 on the cavity's floor, extruded up until it meets the dome's inside,
//! and joined to the dome.
//!
//! brepkit's extrude runs a given distance and has no extrusion up to a face. So the disc is
//! extruded up 19, past the inside sphere (at most 18 high over the disc) and short of the
//! outside one (at least √391 ≈ 19.77 over it): the post's top lies in the dome's wall, and the
//! union leaves the post ending on the dome's inside.
use brepkit_operations::extrude::extrude;
use brepkit_operations::split::split;
use brepkit_topology::builder::{make_circle_edge, make_planar_face_from_wire};
use brepkit_topology::solid::SolidId;
use brepkit_topology::wire::{OrientedEdge, Wire};
use brepkit_topology::Topology;

use crate::solids::contact::{ball, cylinder_z};
use crate::solids::{cut, fuse, p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let plate = cylinder_z(topo, 20.0, (0.0, 0.0), -2.0, 0.0)?;
    let outside = ball(topo, 20.0, (0.0, 0.0, 0.0))?;
    let outside = split(topo, outside, p(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0))?.positive;
    let inside = ball(topo, 18.0, (0.0, 0.0, 0.0))?;
    let inside = split(topo, inside, p(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0))?.positive;
    let dome = fuse(topo, plate, outside)?;
    let dome = cut(topo, dome, inside)?;
    let rim = make_circle_edge(topo, p(0.0, 0.0, 0.0), v(0.0, 0.0, 1.0), 3.0, 1e-7)?;
    let rim = topo.add_wire(Wire::new(vec![OrientedEdge::new(rim, true)], true)?);
    let disc = make_planar_face_from_wire(topo, rim)?;
    let post = extrude(topo, disc, v(0.0, 0.0, 1.0), 19.0)?;
    Ok(fuse(topo, dome, post)?)
}
