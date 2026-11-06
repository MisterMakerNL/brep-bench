//! A gear blank with one tooth space cut by a rack tooth rolled around it.
//!
//! The kernel's sweep of a solid (`sweep_solid_within`): the tooth is its trapezoid extruded from
//! y = −2 to 8, moved, as seen from the blank, by the rack's slide along its pitch line (−20 along
//! z per radian) and then the blank's turn taken back (θ about the y axis, from +x toward +z), so
//! the pitch line rolls without slipping on the pitch circle, for θ from −60° to 60°. Asked for
//! within 22.5 of the blank's axis from y = −1 to 7, which holds the blank; cut from the blank.
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{
    boolean_operation_nary, make_cylinder_brep, sweep_solid_within, BooleanOperation, BrepSolid,
    OutlinePiece, ScrewStep, SolidMotion, SweepClip, SweptTool,
};
use std::f64::consts::PI;

pub fn build() -> Result<BrepSolid, Refusal> {
    let y = p(0.0, 1.0, 0.0);
    let blank = make_cylinder_brep(p(0.0, 0.0, 0.0), y, 22.0, 6.0)?;
    // The trapezoid in the tooth's plane coordinates (x, −z).
    let corners = [[18.0, -0.842856], [24.0, -3.026666], [24.0, 3.026666], [18.0, 0.842856]];
    let outline = (0..4)
        .map(|k| OutlinePiece::Line { from: corners[k], to: corners[(k + 1) % 4] })
        .collect();
    let tooth = SweptTool::Prism {
        origin: p(0.0, 0.0, 0.0),
        normal: y,
        x_axis: p(1.0, 0.0, 0.0),
        outline,
        from: -2.0,
        to: 8.0,
    };
    let rolling = SolidMotion {
        steps: vec![
            ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: p(0.0, 0.0, 1.0), turn: 0.0, advance: -20.0 },
            ScrewStep { axis_origin: p(0.0, 0.0, 0.0), axis_direction: p(0.0, -1.0, 0.0), turn: 1.0, advance: 0.0 },
        ],
        from: -PI / 3.0,
        to: PI / 3.0,
    };
    let around_blank = SweepClip {
        axis_origin: p(0.0, 0.0, 0.0),
        axis_direction: y,
        reference: p(1.0, 0.0, 0.0),
        radius: [15.0, 22.5],
        axial: [-1.0, 7.0],
        angle: None,
    };
    let mut operands = vec![blank];
    operands.extend(sweep_solid_within(&tooth, &rolling, &around_blank)?);
    Ok(boolean_operation_nary(&operands, BooleanOperation::Subtract)?)
}
