//! The common part of two blocks with barrel-vaulted tops, the second the first turned a quarter
//! turn about the z axis: a groin vault. Each block stands on the square −12 ≤ x, y ≤ 12; its top
//! is the rational B-spline of degree 1 along the vault and 2 across it on the control points at
//! heights 10, 26, 10 across, weighted 1, 3/5, 1: an arch of radius 15. Each block is its section
//! — the arch a rational quadratic spline on those control points and weights — extruded 12 to
//! each side: the first's in the yz plane along x, the second's in the xz plane along y, and
//! that one kept only where it meets the first.
use crate::shared::surfaces::conic;
use crate::shared::{Drawing, extrude_symmetric, made, sketch, yz, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

/// The vault's section on a plane, drawn by `at` from (across, up).
fn section(at: impl Fn(f64, f64) -> [f64; 2]) -> Drawing {
    let mut section = Drawing::new();
    let corners = [(-12.0, 0.0), (12.0, 0.0), (12.0, 10.0), (-12.0, 10.0)];
    let [left, right, right_top, left_top] = corners.map(|(a, z)| section.point(at(a, z)));
    section.line(left, right);
    section.line(right, right_top);
    conic(&mut section, right_top, at(0.0, 26.0), left_top, 0.6);
    section.line(left_top, left);
    section
}

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    sketch(b, "along_x", yz(), section(|y, z| [y, z]))?;
    extrude_symmetric(b, "first", "along_x", 24.0, Combine::NewBody)?;
    sketch(b, "along_y", zx(), section(zx_at))?;
    extrude_symmetric(b, "second", "along_y", 24.0, Combine::intersect(made("extrude", "first")))
}
