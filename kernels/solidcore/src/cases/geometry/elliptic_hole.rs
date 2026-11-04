//! The block −12 ≤ x ≤ 12, −8 ≤ y ≤ 8, 0 ≤ z ≤ 8 less the elliptic cylinder x²/36 + y²/9 < 1, run
//! past both faces. ops's profile curves are lines, arcs and NURBS, so the ellipse is the
//! rational quadratic it is.
use crate::bench::cut;
use crate::shared::geometry::elliptic_cylinder;
use crate::Refusal;
use solidcore_ops::{make_box, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([-12.0, -8.0, 0.0], [24.0, 16.0, 8.0])?;
    let hole = elliptic_cylinder(6.0, 3.0, -1.0, 10.0)?;
    cut(block, hole)
}
