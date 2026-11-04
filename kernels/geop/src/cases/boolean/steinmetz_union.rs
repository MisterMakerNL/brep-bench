//! Two cylinders of radius 5 and length 30 crossing at right angles, the first along the x axis
//! from x = −15 to 15, the second along the y axis from y = −15 to 15. Make their union: a circle of
//! radius 5 on the yz plane and one on the xz plane, each extruded 15 each way, and the boolean
//! union of the two.
use crate::shared::booleans::boolean;
use crate::shared::{Drawing, extrude_symmetric, made, sketch, yz, zx};
use crate::{Bench, Refusal};
use geop_ops_model::{BooleanOp, Combine};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut along_x = Drawing::new();
    along_x.circle([0.0, 0.0], 5.0);
    sketch(b, "along_x", yz(), along_x)?;
    extrude_symmetric(b, "first", "along_x", 30.0, Combine::NewBody)?;
    let mut along_y = Drawing::new();
    along_y.circle([0.0, 0.0], 5.0);
    sketch(b, "along_y", zx(), along_y)?;
    extrude_symmetric(b, "second", "along_y", 30.0, Combine::NewBody)?;
    boolean(b, "union", &made("extrude", "first"), &made("extrude", "second"), BooleanOp::Union)
}
