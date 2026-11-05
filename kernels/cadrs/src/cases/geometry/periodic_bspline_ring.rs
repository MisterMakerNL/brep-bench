//! A ring about the z axis, its section a closed, periodic B-spline curve turned a whole turn
//! about the axis. The section, in the xz plane, is the periodic uniform quadratic B-spline whose
//! control polygon is the square of corners (13, 0, 3), (7, 0, 3), (7, 0, −3) and (13, 0, −3):
//! four parabolic arcs, each from the middle of one side to the middle of the next ((13, 0, 0)
//! to (10, 0, 3), and so on), with the square's corner as its control point.
//!
//! cadrs has no periodic B-spline (its B-splines are clamped), and its sketch no quadratic
//! curve, so the section is drawn as a wire on the Front (xz) plane: the curve's four arcs, each
//! the quadratic Bézier from the middle of a side through the corner to the middle of the next.
//! The wire is filled with its plane, and the sheet turned a whole turn about z.
use crate::bench::xz;
use crate::shared::geometry::{turned_about_z, Section};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let section = Section::on(xz(0.0))
        .bezier(&[(13.0, 0.0), (13.0, 3.0), (10.0, 3.0)], &[])
        .bezier(&[(10.0, 3.0), (7.0, 3.0), (7.0, 0.0)], &[])
        .bezier(&[(7.0, 0.0), (7.0, -3.0), (10.0, -3.0)], &[])
        .bezier(&[(10.0, -3.0), (13.0, -3.0), (13.0, 0.0)], &[]);
    turned_about_z(k, &section)
}
