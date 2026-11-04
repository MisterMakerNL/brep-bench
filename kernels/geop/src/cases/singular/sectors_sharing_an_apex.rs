//! Two spherical sectors of a ball of radius 10, each the part within 45° of its axis, both with
//! their apex at the origin: the first about the z axis towards +z, the second the first turned
//! 60° about the y axis; their common part. geop has no cone or ball of its own: the first
//! sector is turned about the z axis from its section in the xz plane, bounded by the z axis
//! from the origin to (0, 0, 10), the arc of radius 10 from there to (10 sin 45°, 0,
//! 10 cos 45°), and the line from there back to the apex. The second is a copy of the first,
//! moved by a turn of 60° about the origin's y axis. Then the boolean intersection.
use crate::shared::booleans::boolean;
use crate::shared::{Drawing, axis, made, revolve, sketch, zx, zx_at};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_model::datums::origin_axis;
use geop_ops_model::{BooleanOp, Combine};
use geop_ops_pattern::{MoveBodyArgs, TurnAxis, body};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let rim = 10.0 * 45f64.to_radians().sin();
    let mut section = Drawing::new();
    let apex = section.point(zx_at(0.0, 0.0));
    let pole = section.point(zx_at(0.0, 10.0));
    let edge = section.point(zx_at(rim, rim));
    section.line(apex, pole);
    // From the pole, counter-clockwise in the sketch (whose second axis is −z), down to the rim.
    section.arc(pole, edge, 45.0);
    section.line(edge, apex);
    sketch(b, "section", zx(), section)?;
    revolve(b, "sector", "section", axis(FrameAxis::Z), 360.0, Combine::NewBody)?;
    let first = made("revolve", "sector");
    b.step(
        "turned",
        MoveBodyArgs {
            bodies: vec![body(&first)],
            translation: [0.0; 3],
            axis: Some(TurnAxis::Datum(origin_axis(FrameAxis::Y))),
            angle: 60.0,
            copy: true,
            combine: Combine::NewBody,
        },
    )?;
    boolean(b, "common", &first, &format!("move(turned,{first})"), BooleanOp::Intersection)
}
