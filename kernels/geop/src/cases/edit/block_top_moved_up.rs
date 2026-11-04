//! A block from the origin to (40, 30, 10), drilled through along z by a hole of radius 5 on the
//! vertical line through (20, 15); then edited directly, its top face moved up 6 to z = 16, the
//! sides and the hole's wall running on up to meet it. The block is extruded, the hole cut by a
//! circle extruded past both faces, and the top, picked by a point on it, moved by geop's direct
//! edit, which rebuilds the solid round the face where it now lies.
use crate::shared::{Drawing, extrude, extrude_symmetric, made, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_edit::MoveFaceArgs;
use geop_ops_model::Combine;
use crate::shared::blends::element;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([0.0, 0.0], [40.0, 30.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "block", "base", 10.0, Combine::NewBody)?;
    let mut bore = Drawing::new();
    bore.circle([20.0, 15.0], 5.0);
    sketch(b, "bore", xy(), bore)?;
    extrude_symmetric(b, "hole", "bore", 40.0, Combine::cut(made("extrude", "block")))?;
    let top = b.face([5.0, 5.0, 10.0])?;
    b.step("raise", MoveFaceArgs { faces: vec![element(top)], translation: [0.0, 0.0, 6.0] })
}
