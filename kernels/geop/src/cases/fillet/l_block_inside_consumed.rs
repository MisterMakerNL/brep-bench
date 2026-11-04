//! An L-shaped block, its section in the xz plane the polygon (0, 0), (10, 0), (10, 3), (3, 3),
//! (3, 10), (0, 10) extruded from y = 0 to y = 10; its inside edge, x = 3, z = 3, rounded to
//! radius 7, adding material, the round consuming both faces beside the edge entirely. The section
//! is sketched on the xz plane and extruded along +y, and the edge filleted.
use crate::shared::{Drawing, extrude, sketch, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_fillet::FilletArgs;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut section = Drawing::new();
    section.polygon(&[
        zx_at(0.0, 0.0),
        zx_at(10.0, 0.0),
        zx_at(10.0, 3.0),
        zx_at(3.0, 3.0),
        zx_at(3.0, 10.0),
        zx_at(0.0, 10.0),
    ]);
    sketch(b, "section", zx(), section)?;
    extrude(b, "block", "section", 10.0, Combine::NewBody)?;
    let inside = b.edge([3.0, 5.0, 3.0])?;
    b.step("round", FilletArgs::constant(vec![inside], 7.0))
}
