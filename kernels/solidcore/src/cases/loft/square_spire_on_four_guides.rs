//! A square spire lofted from the square 18 × 18 at z = 0 to the apex (0, 0, 24), its corners on
//! four cubic Bézier ridges: the one through the corner with x and y positive of control points
//! (9, 9, 0), (15, 15, 8), (12, 12, 19), (0, 0, 24), and its mirror images in the xz and yz
//! planes. Each wall is ruled between two neighbouring ridges (the segment joining their points
//! at each parameter, which share a height), closing to the apex; the floor is flat. solidcore's
//! loft skins through sections and takes no guides, so the walls are the ruled B-spline surfaces
//! between the ridges' control polygons, sewn with the floor into a solid.
use crate::shared::loft::{four_guides, walls};
use crate::shared::nurbs::{polygon, sewn, Face};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let ridges = four_guides(&[[9.0, 9.0, 0.0], [15.0, 15.0, 8.0], [12.0, 12.0, 19.0], [0.0, 0.0, 24.0]]);
    let mut faces = walls(&ridges);
    faces.push(Face::Flat(polygon(&ridges.clone().map(|g| g[0]))));
    sewn(&faces)
}
