//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, less a cylinder of radius 3 on the
//! same axis running exactly from z = 0 to z = 10, its ends in the first's: two circles on the
//! xy plane, each extruded 10 up as a body of its own, and the boolean difference.
use crate::shared::booleans::boolean;
use crate::shared::{Drawing, extrude, made, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_model::{BooleanOp, Combine};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut outer = Drawing::new();
    outer.circle([0.0, 0.0], 5.0);
    sketch(b, "outer", xy(), outer)?;
    extrude(b, "drum", "outer", 10.0, Combine::NewBody)?;
    let mut inner = Drawing::new();
    inner.circle([0.0, 0.0], 3.0);
    sketch(b, "inner", xy(), inner)?;
    extrude(b, "core", "inner", 10.0, Combine::NewBody)?;
    boolean(b, "tube", &made("extrude", "drum"), &made("extrude", "core"), BooleanOp::Difference)
}
