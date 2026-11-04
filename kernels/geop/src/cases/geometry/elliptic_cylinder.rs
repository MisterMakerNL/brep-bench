//! A solid elliptic cylinder: the ellipse of semi-axes 5 along x and 3 along y about the z axis,
//! extruded from z = 0 to z = 10. geop's sketch has no ellipse: the circle of radius 5 is
//! extruded, and the cylinder scaled by 3/5 along y, which makes the circle the ellipse exactly
//! (geop scales the control points).
use crate::shared::{Drawing, extrude, made, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;
use geop_ops_pattern::{ScaleBodyArgs, body};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut circle = Drawing::new();
    circle.circle([0.0, 0.0], 5.0);
    sketch(b, "circle", xy(), circle)?;
    extrude(b, "cylinder", "circle", 10.0, Combine::NewBody)?;
    let cylinder = body(made("extrude", "cylinder"));
    b.step(
        "squash",
        ScaleBodyArgs { bodies: vec![cylinder], about: [0.0; 3], factors: [1.0, 0.6, 1.0] },
    )
}
