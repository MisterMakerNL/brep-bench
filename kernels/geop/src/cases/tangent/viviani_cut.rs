//! A ball of radius 10 centred on the origin and a solid cylinder of radius 5 whose axis is the
//! line x = 5, y = 0, from z = −12 to z = 12, its wall touching the sphere from inside at
//! (10, 0, 0). The cylinder cut from the ball. geop has no ball of its own: the half disc on the side x < 0 of the
//! xy plane, bounded by the y axis, is turned about the y axis, so that the poles are at
//! (0, ±10, 0) and the seam runs through (−10, 0, 0). The cylinder is a circle of radius 5 about
//! (5, 0) on the xy plane extruded from z = −12 to z = 12. geop's cylinder has four seams, where
//! the circle's quarters meet, one of them through (10, 0, 0); it is turned an eighth of a turn
//! about its own axis, its circle's, to keep them off the crossing point. Then the boolean difference.
use crate::shared::booleans::{boolean, extrude_between};
use crate::shared::{Drawing, made, revolve, sketch, sketch_axis, xy};
use crate::{Bench, Refusal};
use geop_ops_model::sketch::sketch_element;
use geop_ops_model::{BooleanOp, Combine};
use geop_ops_pattern::{MoveBodyArgs, TurnAxis, body};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut half = Drawing::new();
    let south = half.point([0.0, -10.0]);
    let north = half.point([0.0, 10.0]);
    let on_axis = half.line(south, north);
    half.arc(north, south, 180.0);
    sketch(b, "half", xy(), half)?;
    revolve(b, "ball", "half", sketch_axis("half", on_axis), 360.0, Combine::NewBody)?;

    let mut round = Drawing::new();
    let rim = round.circle([5.0, 0.0], 5.0);
    sketch(b, "round", xy(), round)?;
    extrude_between(b, "drum", "round", -12.0, 12.0, Combine::NewBody)?;
    b.step(
        "turn",
        MoveBodyArgs {
            bodies: vec![body(made("extrude", "drum"))],
            translation: [0.0; 3],
            axis: Some(TurnAxis::SketchRound(sketch_element("round", rim))),
            angle: 45.0,
            copy: false,
            combine: Combine::NewBody,
        },
    )?;
    boolean(b, "cut", &made("revolve", "ball"), &made("extrude", "drum"), BooleanOp::Difference)
}
