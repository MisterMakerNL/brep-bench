//! An elliptic cylinder: the ellipse x²/25 + y²/9 ≤ 1 in the plane z = 0 (brepkit's closed ellipse
//! edge, semi-axes 5 along x and 3 along y, made a flat face) extruded straight up by 10.
use brepkit_operations::extrude::extrude;
use brepkit_topology::builder::{make_ellipse_edge_with_ref, make_planar_face_from_wire};
use brepkit_topology::solid::SolidId;
use brepkit_topology::wire::{OrientedEdge, Wire};
use brepkit_topology::Topology;

use crate::solids::curved::TOL;
use crate::solids::{p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let rim = make_ellipse_edge_with_ref(
        topo,
        p(0.0, 0.0, 0.0),
        v(0.0, 0.0, 1.0),
        5.0,
        3.0,
        v(1.0, 0.0, 0.0),
        TOL,
    )?;
    let rim = topo.add_wire(Wire::new(vec![OrientedEdge::new(rim, true)], true)?);
    let base = make_planar_face_from_wire(topo, rim)?;
    Ok(extrude(topo, base, v(0.0, 0.0, 1.0), 10.0)?)
}
