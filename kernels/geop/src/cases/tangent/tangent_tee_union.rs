//! Two solid cylinders crossing at right angles: the first of radius 5 along the x axis from
//! x = −15 to 15, the second of radius 3 parallel to the y axis from y = −15 to 15, its axis on
//! x = 0, z = 2, so that both tops are at z = 5 and the walls touch at (0, 0, 5). Their union: a
//! circle of radius 5 on the yz plane and one of radius 3 about (0, 2) on the xz plane, each
//! extruded 15 each way, and the boolean union.
use crate::shared::booleans::boolean;
use crate::shared::{Drawing, extrude_symmetric, made, sketch, yz, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_model::{BooleanOp, Combine};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut main = Drawing::new();
    main.circle([0.0, 0.0], 5.0);
    sketch(b, "main", yz(), main)?;
    extrude_symmetric(b, "first", "main", 30.0, Combine::NewBody)?;
    let mut branch = Drawing::new();
    branch.circle(zx_at(0.0, 2.0), 3.0);
    sketch(b, "branch", zx(), branch)?;
    extrude_symmetric(b, "second", "branch", 30.0, Combine::NewBody)?;
    boolean(b, "union", &made("extrude", "first"), &made("extrude", "second"), BooleanOp::Union)
}
