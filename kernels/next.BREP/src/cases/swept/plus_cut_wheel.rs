//! A wheel cut by a plus-shaped tool turning in mesh with it, as a gear shaper cuts.
//!
//! The kernel's sweep of a solid (`sweep_solid_within`): the tool is the plus extruded from
//! z = −10 to 0, moved, as seen from the wheel, by its own turn t about the z axis and then the
//! wheel's turn taken back (+2t/3 about the vertical line through (−50, 0)), for t from 0° to
//! 540°. Asked for within 40.5 of the wheel's axis from z = −8.5 to −1.5, which holds the wheel;
//! cut from the wheel.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, make_cylinder_brep, sweep_solid_within, BooleanOperation, BrepSolid,
    OutlinePiece, ScrewStep, SolidMotion, SweepClip, SweptTool,
};
use std::f64::consts::PI;

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let wheel = make_cylinder_brep(p(-50.0, 0.0, -8.0), z, 40.0, 6.0)?;
    let corners = [
        [4.0, -4.0], [30.0, -4.0], [30.0, 4.0], [4.0, 4.0], [4.0, 30.0], [-4.0, 30.0],
        [-4.0, 4.0], [-30.0, 4.0], [-30.0, -4.0], [-4.0, -4.0], [-4.0, -30.0], [4.0, -30.0],
    ];
    let outline = (0..12)
        .map(|k| OutlinePiece::Line { from: corners[k], to: corners[(k + 1) % 12] })
        .collect();
    let plus = SweptTool::Prism {
        origin: p(0.0, 0.0, 0.0),
        normal: z,
        x_axis: p(1.0, 0.0, 0.0),
        outline,
        from: -10.0,
        to: 0.0,
    };
    let in_mesh = SolidMotion {
        steps: vec![
            ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: z, turn: 1.0, advance: 0.0 },
            ScrewStep { axis_origin: p(-50.0, 0.0, 0.0), axis_direction: z, turn: 2.0 / 3.0, advance: 0.0 },
        ],
        from: 0.0,
        to: 3.0 * PI,
    };
    let around_wheel = SweepClip {
        axis_origin: p(-50.0, 0.0, 0.0),
        axis_direction: z,
        reference: p(1.0, 0.0, 0.0),
        radius: [10.0, 40.5],
        axial: [-8.5, -1.5],
        angle: None,
    };
    let mut operands = vec![wheel];
    operands.extend(sweep_solid_within(&plus, &in_mesh, &around_wheel)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}
