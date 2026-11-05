//! A picture frame: a rabbeted molding swept around the rectangle with corners (±30, ±20, 0),
//! mitered at its four corners. The section, in (s, z) with s the distance outward from the
//! path, is the polygon (0, 2), (3, 2), (3, 0), (8, 0), (8, 6), (0, 6); each side is the
//! section swept straight along it, cut at each corner by the vertical plane along the
//! rectangle's diagonal there.
use crate::bench::polygon;
use crate::shared::sweep::mitered;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let corners = [[-30.0, -20.0, 0.0], [30.0, -20.0, 0.0], [30.0, 20.0, 0.0], [-30.0, 20.0, 0.0]];
    // counter-clockwise from above, so outward from a run along d is (d_y, −d_x, 0)
    mitered(&corners, true, |p, d| {
        let at = |s: f64, z: f64| [p[0] + s * d[1], p[1] - s * d[0], p[2] + z];
        polygon(&[at(0.0, 2.0), at(3.0, 2.0), at(3.0, 0.0), at(8.0, 0.0), at(8.0, 6.0), at(0.0, 6.0)])
    })
}
