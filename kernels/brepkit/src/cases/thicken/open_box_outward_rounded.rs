//! A sheet of five faces of the cube from the origin to (10, 10, 10), its floor and four walls
//! with no top, thickened by 1 out of the cube, its eight edges rounded by the quarter cylinders
//! of radius 1 about them and its floor's corners by the eighth balls of radius 1 about them.
//!
//! brepkit's thicken takes one face, and its offsets join moved faces only sharp, so the tray is
//! thickened as five faces of the cube by brepkit's shell grown outward (the cube's top its
//! opening), which extends the moved faces until they meet, and the grown tray's eight outer
//! edges, round its floor and up its corners, are then rounded to radius 1 together: each round
//! lies 1 from both moved faces, on the cylinder of radius 1 about the tray's own edge, and the
//! three at a floor corner close on the ball of radius 1 about it.
use brepkit_operations::blend_ops::{all_blended, fillet_v2};
use brepkit_operations::primitives::make_box;
use brepkit_topology::explorer::solid_faces;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::offset::thicken_faces;
use crate::solids::{edges_where, faces_where, near};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let cube = make_box(topo, 10.0, 10.0, 10.0)?;
    let top = faces_where(topo, cube, |q| near(q.z(), 10.0))?;
    let tray: Vec<_> = solid_faces(topo, cube)?
        .into_iter()
        .filter(|f| !top.contains(f))
        .collect();
    let grown = thicken_faces(topo, cube, &tray, 1.0)?;
    let outside = |c: f64| near(c, -1.0) || near(c, 11.0);
    // round the floor, then up the four corners
    let mut edges = edges_where(topo, grown, |q| {
        near(q.z(), -1.0) && (outside(q.x()) || outside(q.y()))
    })?;
    edges.extend(edges_where(topo, grown, |q| {
        outside(q.x()) && outside(q.y())
    })?);
    Ok(all_blended(fillet_v2(topo, grown, &edges, 1.0)?)?)
}
