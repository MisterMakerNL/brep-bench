//! A C-shaped bracket, the section (0, 0), (40, 0), (40, 6), (6, 6), (6, 24), (40, 24), (40, 30),
//! (0, 30) in the xz plane extruded from y = −10 to y = 10, with a hole of radius 4 on the
//! vertical axis through (28, 0) drilled down from the upper arm's top through the upper arm only,
//! ending where it comes out into the gap. The section is sketched on the xz plane and extruded
//! 10 each way; the hole is geop's hole feature on the upper arm's top, at a datum point there,
//! of diameter 8, its end "up to next": up to where it comes out of the solid again.
use crate::shared::blends::{datum_point, element};
use crate::shared::{Drawing, extrude_symmetric, sketch, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_hole::{HoleArgs, HoleCentre, HoleKind, Standard};
use geop_ops_model::datums::datum;
use geop_ops_model::{Combine, Extent};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut section = Drawing::new();
    section.polygon(&[
        zx_at(0.0, 0.0),
        zx_at(40.0, 0.0),
        zx_at(40.0, 6.0),
        zx_at(6.0, 6.0),
        zx_at(6.0, 24.0),
        zx_at(40.0, 24.0),
        zx_at(40.0, 30.0),
        zx_at(0.0, 30.0),
    ]);
    sketch(b, "section", zx(), section)?;
    extrude_symmetric(b, "bracket", "section", 20.0, Combine::NewBody)?;
    datum_point(b, "drill_at", [28.0, 0.0, 30.0])?;
    let top = b.face([20.0, 0.0, 30.0])?;
    b.step(
        "drill",
        HoleArgs {
            face: Some(element(top)),
            points: vec![HoleCentre::Datum(datum("drill_at"))],
            kind: HoleKind::Simple,
            standard: Standard::Custom { diameter: 8.0, head_diameter: 0.0, head_depth: 0.0 },
            end: Extent::UpToNext,
            drill_point: false,
            thread_length: None,
        },
    )
}
