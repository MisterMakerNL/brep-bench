//! A block −12 ≤ x ≤ 12, −8 ≤ y ≤ 8, 0 ≤ z ≤ 8, with the hole x²/36 + y²/9 < 1 bored through it
//! along the z axis. geop's sketch has no ellipse: the tool is the circle of radius 6 extruded
//! from z = −10 to z = 10, past both faces, and scaled by a half along y — exactly the elliptic
//! cylinder (geop scales the control points) — then cut from the block.
use crate::shared::{Drawing, extrude, extrude_symmetric, made, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_boolean::BooleanArgs;
use geop_ops_model::{BooleanOp, Combine, solid};
use geop_ops_pattern::{ScaleBodyArgs, body};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([-12.0, -8.0], [12.0, 8.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "block", "base", 8.0, Combine::NewBody)?;
    let mut circle = Drawing::new();
    circle.circle([0.0, 0.0], 6.0);
    sketch(b, "circle", xy(), circle)?;
    extrude_symmetric(b, "tool", "circle", 20.0, Combine::NewBody)?;
    let tool = made("extrude", "tool");
    b.step(
        "squash",
        ScaleBodyArgs { bodies: vec![body(&tool)], about: [0.0; 3], factors: [1.0, 0.5, 1.0] },
    )?;
    b.step(
        "bore",
        BooleanArgs {
            a: solid(made("extrude", "block")),
            b: solid(tool),
            op: BooleanOp::Difference,
        },
    )
}
