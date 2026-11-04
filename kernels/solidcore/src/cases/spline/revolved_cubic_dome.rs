//! A dome: the region of the xz plane bounded by the z axis from (0, 0, 0) to (0, 0, 10), the x
//! axis from (0, 0, 0) to (9, 0, 0) and the cubic Bézier of control points (9, 0, 0), (12, 0, 4),
//! (9, 0, 10), (0, 0, 10), turned a whole turn about the z axis.
use crate::bench::line;
use crate::shared::geometry::{axis, turned};
use crate::shared::nurbs::bezier_knots;
use crate::Refusal;
use solidcore_ops::sweep::Curve;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let bulge = Curve::Nurbs {
        degree: 3,
        points: vec![[9.0, 0.0, 0.0, 1.0], [12.0, 0.0, 4.0, 1.0], [9.0, 0.0, 10.0, 1.0], [0.0, 0.0, 10.0, 1.0]],
        knots: bezier_knots(3),
    };
    turned(vec![line([0.0, 0.0, 0.0], [9.0, 0.0, 0.0]), bulge, axis(10.0, 0.0)])
}
