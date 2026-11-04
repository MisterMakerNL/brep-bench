//! A block from the origin to (40, 30, 10), the edge where its top meets its back, y = 30,
//! rounded to radius 3; then edited directly, its top moved up 6 to z = 16, carrying the round
//! with it unstretched, the back and the ends running on up to meet them. The block is extruded,
//! the edge filleted, and the top, picked by a point on it, moved by geop's direct edit, which
//! takes along the faces a moved face runs smoothly into: the round.
use crate::shared::{Drawing, extrude, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_edit::MoveFaceArgs;
use geop_ops_fillet::FilletArgs;
use geop_ops_model::Combine;
use crate::shared::blends::element;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([0.0, 0.0], [40.0, 30.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "block", "base", 10.0, Combine::NewBody)?;
    let edge = b.edge([20.0, 30.0, 10.0])?;
    b.step("round", FilletArgs::constant(vec![edge], 3.0))?;
    let top = b.face([20.0, 15.0, 10.0])?;
    b.step("raise", MoveFaceArgs { faces: vec![element(top)], translation: [0.0, 0.0, 6.0] })
}
