//! A picture frame: a rebated moulding swept round the rectangle with corners (±30, ±20, 0),
//! mitred at its four corners. The path is sketched on the xy plane as the rectangle, its side
//! y = −20 drawn as two lines meeting at (0, −20) so that the closed path starts there, where the
//! section is drawn: on the yz plane, the polygon (s, z) = (0, 2), (3, 2), (3, 0), (8, 0), (8, 6),
//! (0, 6) with s the distance outward from the path, at (0, −20 − s, z). Swept round the closed
//! path it makes a ring; geop's sweep mitres two lines meeting at an angle, in the plane bisecting
//! the corner.
use crate::shared::paths::{path2d, sweep};
use crate::shared::{Drawing, sketch, xy, yz};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut path = Drawing::new();
    let p = [[0.0, -20.0], [30.0, -20.0], [30.0, 20.0], [-30.0, 20.0], [-30.0, -20.0]]
        .map(|q| path.point(q));
    for i in 0..5 {
        path.line(p[i], p[(i + 1) % 5]);
    }
    sketch(b, "path", xy(), path)?;

    // On the yz plane, drawn in (y, z): the point (s, z) of the section at (−20 − s, z).
    let mut section = Drawing::new();
    let corners = [(0.0, 2.0), (3.0, 2.0), (3.0, 0.0), (8.0, 0.0), (8.0, 6.0), (0.0, 6.0)];
    section.polygon(&corners.map(|(s, z)| [-20.0 - s, z]));
    sketch(b, "section", yz(), section)?;
    sweep(b, "frame", "section", path2d("path"), Combine::NewBody)
}
