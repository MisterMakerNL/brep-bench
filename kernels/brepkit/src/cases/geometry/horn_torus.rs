//! A horn torus about the z axis: the disc (x − 5)² + z² ≤ 25 of the xz plane (brepkit's closed
//! circle edge about (5, 0, 0), made a flat face), which touches the axis at the origin, turned a
//! whole turn about the z axis. (brepkit's own torus takes only a tube thinner than its middle
//! circle's radius: `make_torus(5, 5)` declines.)
use brepkit_operations::revolve::revolve;
use brepkit_topology::builder::{make_circle_edge_with_ref, make_planar_face_from_wire};
use brepkit_topology::solid::SolidId;
use brepkit_topology::wire::{OrientedEdge, Wire};
use brepkit_topology::Topology;

use crate::solids::curved::TOL;
use crate::solids::{p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let rim = make_circle_edge_with_ref(
        topo,
        p(5.0, 0.0, 0.0),
        v(0.0, 1.0, 0.0),
        5.0,
        v(1.0, 0.0, 0.0),
        TOL,
    )?;
    let rim = topo.add_wire(Wire::new(vec![OrientedEdge::new(rim, true)], true)?);
    let disc = make_planar_face_from_wire(topo, rim)?;
    Ok(revolve(
        topo,
        disc,
        p(0.0, 0.0, 0.0),
        v(0.0, 0.0, 1.0),
        std::f64::consts::TAU,
    )?)
}
