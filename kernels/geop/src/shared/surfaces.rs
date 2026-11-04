//! What the surface cases share: sheets — faces standing on their own — swept from a sketch's
//! curves or spanned by them, and a sheet thickened into a solid.
use crate::{Bench, Refusal};
use geop_core_sketch::CurveId;
use geop_ops_extrude_revolve::{Extents, ExtrudeArgs, Output, RevolveArgs, RevolveAxis};
use geop_ops_model::element;
use geop_ops_model::sketch::{sketch_element, whole_sketch};
use geop_ops_model::sketch3d::CurveSource;
use geop_ops_surface::{BoundarySurfaceArgs, ThickenArgs, ThickenSide};

pub use geop_ops_surface::ThickenSide as Side;

/// Extrudes the curves of the sketch `sketch` `length` along its plane's normal as faces standing
/// on their own (a sheet), as the step `id`.
pub fn sheet_extrude(b: &mut Bench, id: &str, sketch: &str, length: f64) -> Result<(), Refusal> {
    b.step(
        id,
        ExtrudeArgs {
            sketch: whole_sketch(sketch),
            extent: Extents::blind(length),
            output: Output::Faces { up_to: None },
            draft: 0.0,
        },
    )
}

/// Revolves the curves of the sketch `sketch` `degrees` about `axis` as faces standing on their
/// own (a sheet), as the step `id`.
pub fn sheet_revolve(
    b: &mut Bench,
    id: &str,
    sketch: &str,
    axis: RevolveAxis,
    degrees: f64,
) -> Result<(), Refusal> {
    b.step(
        id,
        RevolveArgs {
            sketch: whole_sketch(sketch),
            axis: Some(axis),
            extent: Extents::blind(degrees),
            output: Output::Faces { up_to: None },
        },
    )
}

/// The face standing on its own that fills the closed loop of the curves `curves` of the sketch
/// `sketch` (a boundary surface), as the step `id`: it is named `boundary(id)`.
pub fn fill(b: &mut Bench, id: &str, sketch: &str, curves: &[CurveId]) -> Result<(), Refusal> {
    let edges = curves.iter().map(|&c| CurveSource::Sketch(sketch_element(sketch, c))).collect();
    b.step(id, BoundarySurfaceArgs { edges, tangent: Vec::new() })
}

/// Thickens the sheet the face `face` stands in by `thickness`, on its `side`, as the step `id`:
/// the solid is named `thicken(id)`.
pub fn thicken(
    b: &mut Bench,
    id: &str,
    face: &str,
    thickness: f64,
    side: ThickenSide,
) -> Result<(), Refusal> {
    b.step(id, ThickenArgs { face: element(face), thickness, side })
}

/// Adds to `drawing` the rational quadratic Bézier from `start` to `end` whose middle control
/// point is `control`, weighted `weight` (the ends weighted 1): a conic arc — an ellipse's for a
/// weight under 1, a parabola's for 1, a hyperbola's over 1 — tangent at its ends to the lines to
/// `control`. geop's sketch has no conics; it takes any NURBS as a spline of a given shape.
pub fn conic(
    drawing: &mut super::Drawing,
    start: geop_core_sketch::PointId,
    control: [f64; 2],
    end: geop_core_sketch::PointId,
    weight: f64,
) -> CurveId {
    use geop_core_sketch::{CurveKind, SplineShape};
    let control = drawing.point(control);
    let (zero, one) = (super::n(0.0), super::n(1.0));
    drawing.sketch.add_curve(CurveKind::Spline {
        control_points: vec![start, control, end],
        shape: Some(SplineShape {
            degree: 2,
            knots: vec![zero, zero, zero, one, one, one],
            weights: vec![one, super::n(weight), one],
        }),
    })
}

/// The sheet of the saddle z = 10 + xy/8 over the square −10 ≤ x, y ≤ 10 as the bilinear patch on
/// its four corners: the 3-D sketch `rails` of the closed loop of lines between those corners,
/// and the boundary surface filling it (`boundary(id)`) — four edges, their Coons patch, which
/// for four straight sides is that bilinear patch exactly.
pub fn saddle(b: &mut Bench, id: &str) -> Result<(), Refusal> {
    use geop_core_math::vector::Vector3;
    use geop_ops_model::sketch3d::Sketch3d;
    use geop_ops_sketch3d::AddSketch3dArgs;
    let mut rails = Sketch3d::new();
    let mut at = |p: [f64; 3]| rails.add_point(Vector3::from_array(p.map(super::n)));
    let corners =
        [[-10.0, -10.0, 22.5], [10.0, -10.0, -2.5], [10.0, 10.0, 22.5], [-10.0, 10.0, -2.5]]
            .map(&mut at);
    let sides: Vec<_> = (0..4).map(|k| rails.add_line(corners[k], corners[(k + 1) % 4])).collect();
    b.step("rails", AddSketch3dArgs { sketch: rails, references: Vec::new() })?;
    let edges =
        sides.iter().map(|c| CurveSource::Edge(element(format!("sketch3d(rails,{c})")))).collect();
    b.step(id, BoundarySurfaceArgs { edges, tangent: Vec::new() })
}
