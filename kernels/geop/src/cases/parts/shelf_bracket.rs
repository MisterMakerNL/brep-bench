//! An L-shaped plate bracket 10 thick: its outline (a post 20 wide and 120 tall, an arm 20 deep
//! out to x = 100 along its top, the inside corner rounded to radius 8) sketched on the xz plane
//! and extruded 5 each way along y.
use crate::shared::{Drawing, extrude_symmetric, sketch, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut outline = Drawing::new();
    let p = [
        (0.0, 0.0),
        (20.0, 0.0),
        (20.0, 92.0),
        (28.0, 100.0),
        (100.0, 100.0),
        (100.0, 120.0),
        (0.0, 120.0),
    ]
    .map(|(x, z)| outline.point(zx_at(x, z)));
    outline.line(p[0], p[1]);
    outline.line(p[1], p[2]);
    // About (28, 92): in the sketch (x, −z) from (20, −92) a quarter turn counter-clockwise, past
    // the corner (20, 100), to (28, −100).
    outline.arc(p[2], p[3], 90.0);
    outline.line(p[3], p[4]);
    outline.line(p[4], p[5]);
    outline.line(p[5], p[6]);
    outline.line(p[6], p[0]);
    sketch(b, "outline", zx(), outline)?;
    extrude_symmetric(b, "bracket", "outline", 10.0, Combine::NewBody)
}
