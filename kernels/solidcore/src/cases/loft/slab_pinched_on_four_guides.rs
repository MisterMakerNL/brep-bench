//! A slab lofted through rectangles centred on the z axis, from z = 0 to z = 24, their corners on
//! four cubic Bézier guides: the one through the corner with x and y positive of control points
//! (12, 3.2, 0), (18, −2.8, 8), (15, 0.2, 19), (3, 12.2, 24), and its mirror images in the xz and
//! yz planes. Each wall is ruled between two neighbouring guides (the segment joining their
//! points at each parameter, which share a height); the ends are flat. solidcore's loft skins
//! through sections and takes no guides, so the walls are the ruled B-spline surfaces between
//! the guides' control polygons, sewn with the ends into a solid.
use crate::shared::loft::{four_guides, walls};
use crate::shared::nurbs::{polygon, sewn, Face};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let guides = four_guides(&[[12.0, 3.2, 0.0], [18.0, -2.8, 8.0], [15.0, 0.2, 19.0], [3.0, 12.2, 24.0]]);
    let mut faces = walls(&guides);
    let end = |j: usize| Face::Flat(polygon(&guides.clone().map(|g| g[j])));
    faces.push(end(0));
    faces.push(end(3));
    sewn(&faces)
}
