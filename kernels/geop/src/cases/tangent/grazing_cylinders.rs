//! Two solid cylinders of radius 5 and length 30 crossing at right angles and overlapping to a
//! depth of only 0.01: the first along the x axis from x = −15 to 15, the second parallel to the
//! y axis from y = −15 to 15 with its axis on x = 0, z = 9.99. Their common part: a circle on
//! the yz plane and one about (0, 9.99) on the xz plane, each extruded 15 each way, and the
//! boolean intersection.
use crate::shared::booleans::boolean;
use crate::shared::{Drawing, extrude_symmetric, made, sketch, yz, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_model::{BooleanOp, Combine};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut along_x = Drawing::new();
    along_x.circle([0.0, 0.0], 5.0);
    sketch(b, "along_x", yz(), along_x)?;
    extrude_symmetric(b, "first", "along_x", 30.0, Combine::NewBody)?;
    let mut along_y = Drawing::new();
    along_y.circle(zx_at(0.0, 9.99), 5.0);
    sketch(b, "along_y", zx(), along_y)?;
    extrude_symmetric(b, "second", "along_y", 30.0, Combine::NewBody)?;
    boolean(b, "common", &made("extrude", "first"), &made("extrude", "second"), BooleanOp::Intersection)
}
