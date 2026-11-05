//! A square bar 10 × 10 swept along a path with one sharp corner, from the origin along +x to
//! (40, 0, 0), then turning 120° to the left to (20, 20√3, 0), mitered at the corner: each run
//! the square swept straight along it, cut by the plane through the corner that bisects it,
//! x + √3y = 40.
use crate::bench::polygon;
use crate::shared::sweep::mitered;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let s = 3f64.sqrt();
    mitered(&[[0.0; 3], [40.0, 0.0, 0.0], [20.0, 20.0 * s, 0.0]], false, |p, d| {
        // the square centred on the path, two sides horizontal: across it is (−d_y, d_x, 0)
        let corner = |a: f64, z: f64| [p[0] - a * d[1], p[1] + a * d[0], p[2] + z];
        polygon(&[corner(-5.0, -5.0), corner(5.0, -5.0), corner(5.0, 5.0), corner(-5.0, 5.0)])
    })
}
