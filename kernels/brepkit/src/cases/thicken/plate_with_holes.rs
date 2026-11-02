//! The square in the xy plane from (−10, −10, 0) to (10, 10, 0) with two round holes of radius 3
//! about (−5, 0, 0) and (5, 0, 0), one flat face (normal +z), as a sheet thickened by 1 to each
//! side.
//!
//! brepkit's thicken grows a face to one side only, so the sheet is thickened by 1 along its
//! normal and, again, by 1 against it, and the two slabs fused along the sheet between them.
use brepkit_operations::thicken::thicken;
use brepkit_topology::builder::{make_circle_edge, make_polygon_wire};
use brepkit_topology::face::{Face, FaceSurface};
use brepkit_topology::solid::SolidId;
use brepkit_topology::wire::{OrientedEdge, Wire};
use brepkit_topology::Topology;

use crate::solids::{fuse, p, v};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let corners = [
        p(-10.0, -10.0, 0.0),
        p(10.0, -10.0, 0.0),
        p(10.0, 10.0, 0.0),
        p(-10.0, 10.0, 0.0),
    ];
    let square = make_polygon_wire(topo, &corners, 1e-7)?;
    // each hole runs clockwise seen from +z, against the square
    let mut holes = Vec::new();
    for x in [-5.0, 5.0] {
        let circle = make_circle_edge(topo, p(x, 0.0, 0.0), v(0.0, 0.0, -1.0), 3.0, 1e-7)?;
        holes.push(topo.add_wire(Wire::new(vec![OrientedEdge::new(circle, true)], true)?));
    }
    let plane = FaceSurface::Plane {
        normal: v(0.0, 0.0, 1.0),
        d: 0.0,
    };
    let sheet = topo.add_face(Face::new(square, holes, plane));
    let upper = thicken(topo, sheet, 1.0)?;
    let lower = thicken(topo, sheet, -1.0)?;
    Ok(fuse(topo, upper, lower)?)
}
