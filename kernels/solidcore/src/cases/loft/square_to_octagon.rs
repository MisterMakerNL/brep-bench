//! A square-to-octagon transition: the square −20 ≤ x, y ≤ 20 at z = 0 joined to the octagon of
//! corners (±6, ±14), (±14, ±6) at z = 30 by four flat trapezoids (each side of the square to the
//! octagon's side parallel to it) and four flat triangles (each corner of the square to the
//! octagon's cut side facing it). solidcore's loft pairs the sections' curves one to one, so the
//! ten flat faces are made and sewn into a solid.
use crate::shared::nurbs::{polygon, sewn, Face};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    // the square's corners, and the octagon's, counter-clockwise from the −y side
    let s = |x: f64, y: f64| [x, y, 0.0];
    let o = |x: f64, y: f64| [x, y, 30.0];
    let square = [s(20.0, -20.0), s(20.0, 20.0), s(-20.0, 20.0), s(-20.0, -20.0)];
    let octagon = [
        o(6.0, -14.0),
        o(14.0, -6.0),
        o(14.0, 6.0),
        o(6.0, 14.0),
        o(-6.0, 14.0),
        o(-14.0, 6.0),
        o(-14.0, -6.0),
        o(-6.0, -14.0),
    ];
    let mut faces = vec![Face::Flat(polygon(&square)), Face::Flat(polygon(&octagon))];
    for i in 0..4 {
        let (corner, next) = (square[i], square[(i + 1) % 4]);
        // the octagon's cut side facing this corner, then the side parallel to the square's next
        let (a, b, c) = (octagon[2 * i], octagon[2 * i + 1], octagon[(2 * i + 2) % 8]);
        faces.push(Face::Flat(polygon(&[corner, b, a])));
        faces.push(Face::Flat(polygon(&[corner, next, c, b])));
    }
    sewn(&faces)
}
