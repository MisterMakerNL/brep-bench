//! A sheet, the rectangle in the xy plane from the origin to (10, 8, 0), thickened by 2 to its
//! +z side into the box from the origin to (10, 8, 2). The sheet is the boundary surface that
//! fills the rectangle's four sketched sides, its normal +z; thickened along that normal.
use crate::shared::surfaces::{Side, fill, thicken};
use crate::shared::{Drawing, sketch, xy};
use crate::{Bench, Refusal};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut outline = Drawing::new();
    let sides = outline.rectangle([0.0, 0.0], [10.0, 8.0]);
    sketch(b, "outline", xy(), outline)?;
    fill(b, "sheet", "outline", &sides)?;
    let face = b.face([5.0, 4.0, 0.0])?;
    thicken(b, "slab", &face, 2.0, Side::Along)
}
