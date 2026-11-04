//! A sheet, the whole cylinder of radius 5 about the z axis from z = 0 to z = 10, bounded only by
//! its two circles, thickened by 1 away from the axis: the tube between radii 5 and 6. The sheet
//! is the circle of radius 5 on the xy plane extruded 10 as a face; its normals point away from
//! the axis, so it is thickened along them.
use crate::shared::surfaces::{Side, sheet_extrude, thicken};
use crate::shared::{Drawing, sketch, xy};
use crate::{Bench, Refusal};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut circle = Drawing::new();
    circle.circle([0.0, 0.0], 5.0);
    sketch(b, "circle", xy(), circle)?;
    sheet_extrude(b, "sheet", "circle", 10.0)?;
    let side = b.face([3.0, 4.0, 5.0])?;
    thicken(b, "tube", &side, 1.0, Side::Along)
}
