//! A circle of radius 2 swept a quarter turn along an arc of radius 10 about the z axis, from
//! (10, 0, 0) to (0, 10, 0): the arc sketched on the xy plane, the circle on the xz plane, square
//! to the arc where it starts, and swept along it.
use crate::shared::paths::{path2d, sweep};
use crate::shared::{Drawing, sketch, xy, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut arc = Drawing::new();
    let (start, end) = (arc.point([10.0, 0.0]), arc.point([0.0, 10.0]));
    arc.arc(start, end, 90.0);
    sketch(b, "arc", xy(), arc)?;

    let mut section = Drawing::new();
    section.circle(zx_at(10.0, 0.0), 2.0);
    sketch(b, "section", zx(), section)?;
    sweep(b, "pipe", "section", path2d("arc"), Combine::NewBody)
}
