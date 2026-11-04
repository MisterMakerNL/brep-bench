//! A lemon: the circular segment of the disc of radius 5 about (−3, 0) in the xz plane with x ≥ 0
//! — its arc from (0, −4) through (2, 0) to (0, 4), its chord on the z axis — turned a whole turn
//! about the z axis. The segment is sketched, the chord a line, and revolved about that line.
use crate::shared::{Drawing, revolve, sketch, sketch_axis, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut segment = Drawing::new();
    let (bottom, top) = (segment.point(zx_at(0.0, -4.0)), segment.point(zx_at(0.0, 4.0)));
    let chord = segment.line(bottom, top);
    // About (−3, 0), from the top through (2, 0) to the bottom: counter-clockwise in the sketch,
    // whose second axis is −z, by 2·acos(3/5).
    segment.arc(top, bottom, 2.0 * 0.6f64.acos().to_degrees());
    sketch(b, "segment", zx(), segment)?;
    revolve(b, "lemon", "segment", sketch_axis("segment", chord), 360.0, Combine::NewBody)
}
