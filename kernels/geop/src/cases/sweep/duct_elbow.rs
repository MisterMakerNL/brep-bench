//! A hollow square duct, 18 × 18 with a 14 × 14 hole, bent a quarter turn round an arc of radius
//! 30 about the z axis from (30, 0, 0) to (0, 30, 0): the arc sketched on the xy plane, the hollow
//! square on the xz plane, centred on the arc where it starts, and swept along it.
use crate::shared::paths::{path2d, sweep};
use crate::shared::{Drawing, sketch, xy, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut arc = Drawing::new();
    let (start, end) = (arc.point([30.0, 0.0]), arc.point([0.0, 30.0]));
    arc.arc(start, end, 90.0);
    sketch(b, "centreline", xy(), arc)?;

    let mut section = Drawing::new();
    section.rectangle(zx_at(21.0, -9.0), zx_at(39.0, 9.0));
    section.rectangle(zx_at(23.0, -7.0), zx_at(37.0, 7.0));
    sketch(b, "section", zx(), section)?;
    sweep(b, "duct", "section", path2d("centreline"), Combine::NewBody)
}
