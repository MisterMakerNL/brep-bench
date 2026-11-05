//! A pipe of radius 2 swept along three edges of the cube of side 30, from the origin to
//! (30, 0, 0), to (30, 30, 0), to (30, 30, 30), mitered at its two corners: each run the disk
//! swept straight along it, cut by the plane through the corner square to the sum of the two
//! runs' directions (x + y = 30, then y + z = 30).
use crate::bench::circle;
use crate::shared::sweep::mitered;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let corners = [[0.0; 3], [30.0, 0.0, 0.0], [30.0, 30.0, 0.0], [30.0, 30.0, 30.0]];
    mitered(&corners, false, |p, d| {
        // each run lies along an axis: the next axis round is square to it
        let reference = [d[2].abs(), d[0].abs(), d[1].abs()];
        vec![circle(p, d, reference, 2.0)]
    })
}
