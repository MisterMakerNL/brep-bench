//! A block −12 ≤ x ≤ 12, −8 ≤ y ≤ 8, 0 ≤ z ≤ 8 less an elliptic cylinder along the z axis: the
//! ellipse x²/36 + y²/9 ≤ 1 (brepkit's closed ellipse edge, semi-axes 6 along x and 3 along y, made
//! a flat face at z = −1) extruded up to z = 9, past both faces of the block.
use brepkit_operations::extrude::extrude;
use brepkit_operations::primitives::make_box;
use brepkit_topology::builder::{make_ellipse_edge_with_ref, make_planar_face_from_wire};
use brepkit_topology::solid::SolidId;
use brepkit_topology::wire::{OrientedEdge, Wire};
use brepkit_topology::Topology;

use crate::solids::curved::TOL;
use crate::solids::{cut, moved, p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let block = make_box(topo, 24.0, 16.0, 8.0)?;
    let block = moved(topo, block, -12.0, -8.0, 0.0)?;
    let rim = make_ellipse_edge_with_ref(
        topo,
        p(0.0, 0.0, -1.0),
        v(0.0, 0.0, 1.0),
        6.0,
        3.0,
        v(1.0, 0.0, 0.0),
        TOL,
    )?;
    let rim = topo.add_wire(Wire::new(vec![OrientedEdge::new(rim, true)], true)?);
    let end = make_planar_face_from_wire(topo, rim)?;
    let bore = extrude(topo, end, v(0.0, 0.0, 1.0), 10.0)?;
    Ok(cut(topo, block, bore)?)
}
