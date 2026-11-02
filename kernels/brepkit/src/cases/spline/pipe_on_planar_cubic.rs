//! A fat pipe round a cubic Bézier bend: the disc of radius 20 centred on the curve's start, square
//! to it there (brepkit's circle edge about the start, normal to the curve's first tangent, made a
//! flat face), swept by brepkit's `sweep` along the cubic Bézier on the case's four control points
//! (brepkit's NURBS). `sweep` carries the section on rotation-minimizing frames, so it stays square
//! to the curve, and closes the ends with flat discs.
use brepkit_operations::sweep::sweep;
use brepkit_topology::builder::{make_circle_edge, make_planar_face_from_wire};
use brepkit_topology::solid::SolidId;
use brepkit_topology::wire::{OrientedEdge, Wire};
use brepkit_topology::Topology;

use crate::solids::curved::{bezier, TOL};
use crate::solids::p;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let path = bezier(&[
        p(0.0, 0.0, 0.0),
        p(12.0, -16.0, 0.0),
        p(32.0, -16.0, 0.0),
        p(44.0, 0.0, 0.0),
    ])?;
    let start = path.evaluate(0.0);
    let rim = make_circle_edge(topo, start, path.tangent(0.0)?, 20.0, TOL)?;
    let rim = topo.add_wire(Wire::new(vec![OrientedEdge::new(rim, true)], true)?);
    let section = make_planar_face_from_wire(topo, rim)?;
    Ok(sweep(topo, section, &path)?)
}
