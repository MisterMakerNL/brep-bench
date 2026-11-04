//! An L-block, the section (0, 0), (40, 0), (40, 10), (15, 10), (15, 30), (0, 30) in the xz plane
//! extruded from y = 0 to y = 20; then edited directly, its riser, the face x = 15, moved out 10
//! along +x to x = 25, the faces beside it following. The section is sketched on the xz plane and
//! extruded along +y, and the riser, picked by a point on it, moved by geop's direct edit.
use crate::shared::{Drawing, extrude, sketch, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_edit::MoveFaceArgs;
use geop_ops_model::Combine;
use crate::shared::blends::element;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut section = Drawing::new();
    section.polygon(&[
        zx_at(0.0, 0.0),
        zx_at(40.0, 0.0),
        zx_at(40.0, 10.0),
        zx_at(15.0, 10.0),
        zx_at(15.0, 30.0),
        zx_at(0.0, 30.0),
    ]);
    sketch(b, "section", zx(), section)?;
    extrude(b, "block", "section", 20.0, Combine::NewBody)?;
    let riser = b.face([15.0, 10.0, 20.0])?;
    b.step("move", MoveFaceArgs { faces: vec![element(riser)], translation: [10.0, 0.0, 0.0] })
}
