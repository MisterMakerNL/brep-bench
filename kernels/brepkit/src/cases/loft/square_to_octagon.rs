//! A square-to-octagon transition. brepkit's loft joins its sections vertex to vertex (resampling
//! the square to the octagon's eight), so it cannot pair a corner of the square with a whole side
//! of the octagon. The solid is the convex hull of the two, as the description says, so it is
//! brepkit's convex hull of the square's four corners at z = 0 and the octagon's eight at z = 30,
//! whose faces are the floor, the top, the four trapezoids and the four triangles.
use brepkit_operations::primitives::make_convex_hull;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::p;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let square = [(20.0, -20.0), (20.0, 20.0), (-20.0, 20.0), (-20.0, -20.0)];
    let octagon = [
        (6.0, -14.0),
        (14.0, -6.0),
        (14.0, 6.0),
        (6.0, 14.0),
        (-6.0, 14.0),
        (-14.0, 6.0),
        (-14.0, -6.0),
        (-6.0, -14.0),
    ];
    let corners: Vec<_> = square
        .iter()
        .map(|&(x, y)| p(x, y, 0.0))
        .chain(octagon.iter().map(|&(x, y)| p(x, y, 30.0)))
        .collect();
    Ok(make_convex_hull(topo, &corners)?)
}
