//! A prism standing on the xy plane, from z = 0 to z = 10, whose section is the closed, periodic
//! uniform quadratic B-spline whose control polygon is the square of corners (10, 10),
//! (−10, 10), (−10, −10) and (10, −10): four parabolic arcs, each from the middle of one side to
//! the middle of the next, (10, 0) to (0, 10) and so on, with the square's corner as its control
//! point. The prism's side is that curve swept straight up; its ends are flat.
//!
//! cadrs has no periodic B-spline (its B-splines are clamped), and its sketch no quadratic
//! curve, so the section is drawn as a wire on the Top plane: the curve's four arcs, each the
//! quadratic Bézier from the middle of a side through the corner to the middle of the next. The
//! wire is filled with its plane, and the sheet slid 10 up.
use crate::bench::{v3, xy};
use crate::shared::geometry::Section;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let section = Section::on(xy(0.0))
        .bezier(&[(10.0, 0.0), (10.0, 10.0), (0.0, 10.0)], &[])
        .bezier(&[(0.0, 10.0), (-10.0, 10.0), (-10.0, 0.0)], &[])
        .bezier(&[(-10.0, 0.0), (-10.0, -10.0), (0.0, -10.0)], &[])
        .bezier(&[(0.0, -10.0), (10.0, -10.0), (10.0, 0.0)], &[]);
    section.slid(k, v3(0.0, 0.0, 10.0))
}
