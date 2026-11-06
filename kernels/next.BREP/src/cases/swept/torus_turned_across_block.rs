//! A curved block grooved by a leaning torus cutter turned past it about the block's own axis.
//!
//! The kernel's sweep of a solid (`sweep_solid_within`): the cutter is a solid of revolution (its
//! meridian the tube's circle, radius 2 at 2.5 from its axis), turned about the z axis from −140°
//! to +140°. Asked for between 7.5 and 12.5 from the z axis, from z = −1.5 to 1.5 and from −10°
//! to 70°, which holds the block and which neither end position reaches; cut from the block.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, make_line, revolve_profile_brep, sweep_solid_within, BooleanOperation,
    BrepSolid, OutlinePiece, ScrewStep, SolidMotion, SweepClip, SweptTool,
};
use std::f64::consts::TAU;

pub fn build() -> Result<BrepSolid, Refusal> {
    let z = p(0.0, 0.0, 1.0);
    let section = [p(8.0, 0.0, -1.0), p(12.0, 0.0, -1.0), p(12.0, 0.0, 1.0), p(8.0, 0.0, 1.0)];
    let mut outline = Vec::new();
    for k in 0..4 {
        outline.push(make_line(section[k], section[(k + 1) % 4])?);
    }
    let block = revolve_profile_brep(&outline, p(0.0, 0.0, 0.0), z, 60.0f64.to_radians())?;
    let lean = 25.0f64.to_radians();
    let cutter = SweptTool::Revolved {
        axis_origin: p(5.0, 0.0, 0.0),
        axis_direction: p(0.0, lean.sin(), lean.cos()),
        radial: p(1.0, 0.0, 0.0),
        meridian: vec![OutlinePiece::Arc { centre: [2.5, 0.0], radius: 2.0, start: 0.0, end: TAU }],
    };
    let turn = SolidMotion {
        steps: vec![ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: z, turn: 1.0, advance: 0.0 }],
        from: (-140.0f64).to_radians(),
        to: 140.0f64.to_radians(),
    };
    let around_block = SweepClip {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: z,
        reference: p(1.0, 0.0, 0.0),
        radius: [7.5, 12.5],
        axial: [-1.5, 1.5],
        angle: Some([(-10.0f64).to_radians(), 70.0f64.to_radians()]),
    };
    let mut operands = vec![block];
    operands.extend(sweep_solid_within(&cutter, &turn, &around_block)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}
