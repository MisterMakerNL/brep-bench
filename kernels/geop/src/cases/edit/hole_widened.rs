//! A plate from the origin to (40, 30, 8), drilled through along z by a hole of radius 5 on the
//! vertical line through (20, 15); then edited directly, the hole's wall offset 2 into the plate,
//! to radius 7, the top and bottom trimmed back to meet it. The plate is extruded, the hole cut by
//! a circle extruded past both faces, and the wall, picked by a point on it, offset by geop's
//! direct edit: −2 along its outward normal, which points into the hole, is 2 into the plate.
use crate::shared::{Drawing, extrude, extrude_symmetric, made, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_edit::OffsetFaceArgs;
use geop_ops_model::Combine;
use crate::shared::blends::element;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([0.0, 0.0], [40.0, 30.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "plate", "base", 8.0, Combine::NewBody)?;
    let mut bore = Drawing::new();
    bore.circle([20.0, 15.0], 5.0);
    sketch(b, "bore", xy(), bore)?;
    extrude_symmetric(b, "hole", "bore", 40.0, Combine::cut(made("extrude", "plate")))?;
    let wall = b.face([23.0, 19.0, 4.0])?;
    b.step("widen", OffsetFaceArgs { faces: vec![element(wall)], distance: -2.0 })
}
