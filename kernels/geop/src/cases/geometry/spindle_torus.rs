//! A solid spindle torus about the z axis: a tube of radius 5 about a middle circle of radius 3,
//! counted once — the turn of the part of the disc (x − 3)² + z² ≤ 25 of the xz plane with x ≥ 0,
//! an apple dimpled where its surface meets the axis at (0, 0, ±4). geop has no torus, and a disc
//! that crosses the axis does not revolve: the part with x ≥ 0 — the tube's outer arc from
//! (0, 4) through (8, 0) to (0, −4), closed by the axis — is revolved a whole turn about that
//! line.
use crate::shared::{Drawing, revolve, sketch, sketch_axis, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut apple = Drawing::new();
    let (bottom, top) = (apple.point(zx_at(0.0, -4.0)), apple.point(zx_at(0.0, 4.0)));
    let on_axis = apple.line(bottom, top);
    // About (3, 0), from the top through (8, 0) to the bottom: counter-clockwise in the sketch,
    // whose second axis is −z, by all of the turn but the part the axis cuts off, 2·acos(3/5).
    apple.arc(top, bottom, 360.0 - 2.0 * 0.6f64.acos().to_degrees());
    sketch(b, "apple", zx(), apple)?;
    revolve(b, "spindle", "apple", sketch_axis("apple", on_axis), 360.0, Combine::NewBody)
}
