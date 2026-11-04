//! The drum of `geometry/saddle-drum` bored through along its axis: every point with
//! 9 ≤ x² + y² ≤ 64 and 0 ≤ z ≤ 10 + xy/8, its top the saddle z = 10 + xy/8 made as the bilinear
//! B-spline patch on the corners of the square −10 ≤ x, y ≤ 10 at the saddle's heights there.
//! The patch is the boundary surface filling the loop of its four sides, lines of a 3-D sketch:
//! their Coons patch, which is the bilinear patch exactly. The ring between the circles of
//! radius 3 and 8, extruded from z = 0 to z = 30, past the saddle, is split along it and the
//! piece above deleted.
use crate::shared::surfaces::saddle;
use crate::shared::{Drawing, extrude, made, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_boolean::SplitArgs;
use geop_ops_edit::DeleteBodyArgs;
use geop_ops_model::{Combine, element, solid};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    saddle(b, "saddle")?;
    let mut circle = Drawing::new();
    circle.circle([0.0, 0.0], 8.0);
    circle.circle([0.0, 0.0], 3.0);
    sketch(b, "circle", xy(), circle)?;
    extrude(b, "drum", "circle", 30.0, Combine::NewBody)?;
    let drum = solid(made("extrude", "drum"));
    b.step("cut", SplitArgs { solid: drum, face: element("boundary(saddle)") })?;
    // The piece above: the one with the cylinder's top, its end.
    b.step("off", DeleteBodyArgs { bodies: vec![element("extrude(drum,end)")] })
}
