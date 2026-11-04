//! A ball of radius 5 about the origin whose surface is the standard biquadratic NURBS sphere:
//! rational quadratic round the axis, nine control points on quarter-turn knots, and along the
//! meridian the half circle of five control points from pole to pole. geop takes no surface by
//! its control net. Its revolve builds exactly that net from a profile: the half disc of the xz
//! plane, its half circle from (0, 5) through (5, 0) to (0, −5) — geop's arc, a rational
//! quadratic in two quarter turns, the five control points — revolved a whole turn about the z
//! axis, rational quadratic in quarter turns; geop makes it four faces, a quarter turn each.
use crate::shared::{Drawing, revolve, sketch, sketch_axis, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut half = Drawing::new();
    let (south, north) = (half.point(zx_at(0.0, -5.0)), half.point(zx_at(0.0, 5.0)));
    let on_axis = half.line(south, north);
    half.arc(north, south, 180.0);
    sketch(b, "half", zx(), half)?;
    revolve(b, "ball", "half", sketch_axis("half", on_axis), 360.0, Combine::NewBody)
}
