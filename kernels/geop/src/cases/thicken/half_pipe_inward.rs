//! A sheet, the half of the cylinder of radius 5 about the z axis where y ≥ 0, from z = 0 to
//! z = 10, thickened by 2 toward the axis: the half tube between radii 3 and 5. The sheet is
//! the half circle of radius 5 on the xy plane, from (5, 0) through (0, 5) to (−5, 0), extruded
//! 10 as a face; its normals point away from the axis, so it is thickened behind them.
use crate::shared::surfaces::{Side, sheet_extrude, thicken};
use crate::shared::{Drawing, sketch, xy};
use crate::{Bench, Refusal};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut half = Drawing::new();
    let (right, left) = (half.point([5.0, 0.0]), half.point([-5.0, 0.0]));
    half.arc(right, left, 180.0);
    sketch(b, "half", xy(), half)?;
    sheet_extrude(b, "sheet", "half", 10.0)?;
    let side = b.face([3.0, 4.0, 5.0])?;
    thicken(b, "pipe", &side, 2.0, Side::Against)
}
