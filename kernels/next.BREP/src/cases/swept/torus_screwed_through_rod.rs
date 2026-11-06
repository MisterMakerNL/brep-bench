//! A rod grooved by a torus wheel carried along a screw about its axis, as a thread is ground.
//!
//! The kernel's sweep of a solid (`sweep_solid_within`): the wheel is a solid of revolution (its
//! meridian the tube's circle, radius 2 at 6 from its axis), swept by the screw about the z axis
//! that rises 30 a turn, from a turn of −108° to +348°. Asked for inside the rod's cylinder of
//! radius 5.5 from z = −1 to 21, where neither end position reaches; cut from the rod.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, make_cylinder_brep, sweep_solid_within, BooleanOperation, BrepSolid,
    OutlinePiece, ScrewStep, SolidMotion, SweepClip, SweptTool,
};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, Refusal> {
    let rod = make_cylinder_brep(p(0.0, 0.0, 0.0), p(0.0, 0.0, 1.0), 5.0, 20.0)?;
    let h = (TAU * 5.5 / 30.0).atan();
    let wheel = SweptTool::Revolved {
        axis_origin: p(11.5, 0.0, 0.0),
        axis_direction: p(0.0, h.cos(), -h.sin()),
        radial: p(-1.0, 0.0, 0.0),
        meridian: vec![OutlinePiece::Arc { centre: [6.0, 0.0], radius: 2.0, start: 0.0, end: TAU }],
    };
    let screw = SolidMotion {
        steps: vec![ScrewStep {
            axis_origin: p(0.0, 0.0, 0.0),
            axis_direction: p(0.0, 0.0, 1.0),
            turn: 1.0,
            advance: 30.0 / TAU,
        }],
        from: (-108.0f64).to_radians(),
        to: 348.0f64.to_radians(),
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
