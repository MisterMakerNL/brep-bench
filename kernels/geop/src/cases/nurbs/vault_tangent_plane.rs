//! A block on the square −12 ≤ x, y ≤ 12 with a barrel-vaulted top — the rational B-spline of
//! degree 1 along x and 2 along y on the control points (x, −12, 10), (x, 0, 26), (x, 12, 10),
//! weighted 1, 3/5, 1, an arch of radius 15 about the line y = 0, z = 1 — cut by a box whose
//! floor, the plane z = 16, touches the vault along its crown. The block is its section in the yz
//! plane, the vault's arch a rational quadratic spline on those control points and weights,
//! extruded 12 to each side along x: the extrusion is exactly that surface. The box, from
//! z = 16 to z = 30 and reaching past the block, is cut from it.
use crate::shared::surfaces::conic;
use crate::shared::{Drawing, extrude, extrude_symmetric, made, sketch, yz};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_model::Combine;
use geop_ops_model::datums::{datum, origin_plane};
use geop_ops_model::selection::Selected;
use geop_ops_model::sketch::SketchPlane;
use geop_ops_reference_geometry::{AddDatumArgs, Construction};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut section = Drawing::new();
    let [left, right, right_top, left_top] =
        [[-12.0, 0.0], [12.0, 0.0], [12.0, 10.0], [-12.0, 10.0]].map(|p| section.point(p));
    section.line(left, right);
    section.line(right, right_top);
    conic(&mut section, right_top, [0.0, 26.0], left_top, 0.6);
    section.line(left_top, left);
    sketch(b, "section", yz(), section)?;
    extrude_symmetric(b, "vault", "section", 24.0, Combine::NewBody)?;

    let xy_plane = Selected::Datum(datum(origin_plane(FrameAxis::Z).name()));
    let construction = Construction::Offset { distance: 16.0.into() };
    b.step("crown", AddDatumArgs { selection: vec![xy_plane], construction })?;
    let mut lid = Drawing::new();
    lid.rectangle([-20.0, -20.0], [20.0, 20.0]);
    sketch(b, "lid", SketchPlane::Datum(datum("crown")), lid)?;
    extrude(b, "cut", "lid", 14.0, Combine::cut(made("extrude", "vault")))
}
