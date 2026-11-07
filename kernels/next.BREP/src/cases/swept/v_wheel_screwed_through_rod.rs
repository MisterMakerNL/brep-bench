//! A rod threaded by a wheel with a sharp V rim, carried along a screw about its axis.
//!
//! The kernel's sweep of a solid (`sweep_solid_within`): the wheel is the solid of revolution of
//! its triangle (tip 10 from its axis, the other corners at 7.5 and ±2.5·tan 30° along it), swept
//! by the screw about the z axis that rises 30 a turn, from a turn of −144° to +384°. Asked for
//! inside the rod's cylinder of radius 5.5 from z = −1 to 21, where neither end position reaches;
//! cut from the rod.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, make_cylinder_brep, sweep_solid_within, BooleanOperation, BrepSolid,
    OutlinePiece, ScrewStep, SolidMotion, SweepClip, SweptTool,
};
use std::f64::consts::{PI, TAU};

pub fn build() -> Result<BrepSolid, Refusal> {
    let rod = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 20.0)?;
    let h = (TAU * 4.5 / 30.0).atan();
    let flank = 2.5 * (PI / 6.0).tan();
    let (tip, upper, lower) = ([10.0, 0.0], [7.5, flank], [7.5, -flank]);
    let wheel = SweptTool::Revolved {
        axis_origin: p(14.0, 0.0, 0.0),
        axis_direction: p(0.0, h.cos(), -h.sin()),
        radial: p(-1.0, 0.0, 0.0),
        meridian: vec![
            OutlinePiece::Line { from: tip, to: upper },
            OutlinePiece::Line { from: upper, to: lower },
            OutlinePiece::Line { from: lower, to: tip },
        ],
    };
    let screw = SolidMotion {
        steps: vec![ScrewStep {
            axis_origin: p(0.0, 0.0, 0.0),
            axis_direction: p(0.0, 0.0, 1.0),
            turn: 1.0,
            advance: 30.0 / TAU,
        }],
        from: (-144.0f64).to_radians(),
        to: 384.0f64.to_radians(),
    };
    let around_rod = SweepClip {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: p(0.0, 0.0, 1.0),
        reference: p(1.0, 0.0, 0.0),
        radius: [0.0, 5.5],
        axial: [-1.0, 21.0],
        angle: None,
    };
    let mut operands = vec![rod];
    operands.extend(sweep_solid_within(&wheel, &screw, &around_rod)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}
