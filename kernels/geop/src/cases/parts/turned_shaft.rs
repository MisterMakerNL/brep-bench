//! A stepped shaft on the z axis: a cylinder of radius 20 from z = 0 to 60 and one of radius 12
//! on to z = 160, the inside corner at the shoulder rounded to radius 4 — its section sketched on
//! the xz plane, its straight side on the z axis, and turned a full turn about that side.
use crate::shared::{Drawing, revolve, sketch, sketch_axis, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut section = Drawing::new();
    let p = [
        (0.0, 0.0),
        (20.0, 0.0),
        (20.0, 60.0),
        (16.0, 60.0),
        (12.0, 64.0),
        (12.0, 160.0),
        (0.0, 160.0),
    ]
    .map(|(x, z)| section.point(zx_at(x, z)));
    section.line(p[0], p[1]);
    section.line(p[1], p[2]);
    section.line(p[2], p[3]);
    // About (16, 64): in the sketch (x, −z) from (16, −60) a quarter turn counter-clockwise, past
    // the corner (12, 60), to (12, −64).
    section.arc(p[3], p[4], 90.0);
    section.line(p[4], p[5]);
    section.line(p[5], p[6]);
    let on_axis = section.line(p[6], p[0]);
    sketch(b, "section", zx(), section)?;
    revolve(b, "shaft", "section", sketch_axis("section", on_axis), 360.0, Combine::NewBody)
}
