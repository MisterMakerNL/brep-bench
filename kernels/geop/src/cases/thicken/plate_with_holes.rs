//! A sheet, the square in the xy plane from (−10, −10, 0) to (10, 10, 0) with two round holes of
//! radius 3 through it about (−5, 0, 0) and (5, 0, 0), thickened by 1 to each side: the plate
//! from z = −1 to z = 1, holed right through. geop's boundary surface fills one loop, not a loop
//! with holes; so the sheet is the face of the sketched region (the square less the circles)
//! extruded into a thin plate, extracted into a face standing on its own, the plate deleted.
use crate::shared::surfaces::{Side, thicken};
use crate::shared::{Drawing, extrude, made, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_edit::{DeleteBodyArgs, ExtractFaceArgs};
use geop_ops_model::{Combine, element};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut outline = Drawing::new();
    outline.rectangle([-10.0, -10.0], [10.0, 10.0]);
    outline.circle([-5.0, 0.0], 3.0);
    outline.circle([5.0, 0.0], 3.0);
    sketch(b, "outline", xy(), outline)?;
    extrude(b, "blank", "outline", -1.0, Combine::NewBody)?;
    // The plate's face on the sketch's plane: the extrude's start.
    b.step("sheet", ExtractFaceArgs { face: element("extrude(blank,start)") })?;
    b.step("scrap", DeleteBodyArgs { bodies: vec![element(made("extrude", "blank"))] })?;
    let face = b.face([0.0, 5.0, 0.0])?;
    thicken(b, "plate", &face, 2.0, Side::Both)
}
