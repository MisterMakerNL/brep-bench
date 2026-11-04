//! What the geop implementations of brep-bench share: the origin's planes and axes, sketches
//! drawn at given coordinates, and the steps written briefly that nearly every case takes —
//! a sketch added on a plane, extruded or revolved.
use crate::{Bench, Refusal};
use geop_core_math::{primitives::FrameAxis, scalars::Scalar};
use geop_core_sketch::{CurveId, PointId};
use geop_ops_extrude_revolve::{Extents, ExtrudeArgs, Output, RevolveArgs, RevolveAxis};
use geop_ops_model::Combine;
use geop_ops_model::datums::{origin_axis, origin_plane};
use geop_ops_model::sketch::{Sketch, SketchPlane, sketch_element, whole_sketch};
use geop_ops_sketch::AddSketchArgs;

/// A number, as design data.
pub fn n(x: f64) -> Scalar {
    Scalar::from_f64(x)
}

/// The origin's xy plane, normal to z: a sketch on it is drawn in `(x, y)`.
pub fn xy() -> SketchPlane {
    SketchPlane::Datum(origin_plane(FrameAxis::Z))
}

/// The origin's yz plane, normal to x: a sketch on it is drawn in `(y, z)`.
pub fn yz() -> SketchPlane {
    SketchPlane::Datum(origin_plane(FrameAxis::X))
}

/// The origin's zx plane, normal to y: a sketch on it is drawn in `(x, −z)` (see [`zx_at`]).
pub fn zx() -> SketchPlane {
    SketchPlane::Datum(origin_plane(FrameAxis::Y))
}

/// The point `(x, z)` of the xz plane, in the coordinates a sketch on [`zx`] is drawn in: its
/// first axis is x and its second −z (the plane's normal is y = x × −z). So the sketch sees the
/// xz plane from +y, and an arc turning counter-clockwise in it turns clockwise as the xz plane
/// is drawn with x to the right and z up.
pub fn zx_at(x: f64, z: f64) -> [f64; 2] {
    [x, -z]
}

/// The origin's axis `axis`, as a line to revolve about.
pub fn axis(axis: FrameAxis) -> RevolveAxis {
    RevolveAxis::Datum(origin_axis(axis))
}

/// A sketch, drawn at given coordinates: every point where it is put, nothing constrained.
#[derive(Default)]
pub struct Drawing {
    pub sketch: Sketch,
}

impl Drawing {
    pub fn new() -> Self {
        Self::default()
    }

    /// A point at `at`.
    pub fn point(&mut self, at: [f64; 2]) -> PointId {
        self.sketch.new_point_at(at)
    }

    /// The line from `a` to `b`.
    pub fn line(&mut self, a: PointId, b: PointId) -> CurveId {
        self.sketch.add_line(a, b)
    }

    /// The arc from `a` to `b` turning counter-clockwise by `degrees` (clockwise if negative).
    pub fn arc(&mut self, a: PointId, b: PointId, degrees: f64) -> CurveId {
        self.sketch.add_arc(a, b, n(degrees.to_radians()))
    }

    /// The circle of `radius` about `center`.
    pub fn circle(&mut self, center: [f64; 2], radius: f64) -> CurveId {
        let c = self.point(center);
        self.sketch.add_circle(c, n(radius))
    }

    /// The closed polygon through `corners`: its corners and its sides, the side from corner
    /// `i` to corner `i + 1` first.
    pub fn polygon(&mut self, corners: &[[f64; 2]]) -> (Vec<PointId>, Vec<CurveId>) {
        let points: Vec<PointId> = corners.iter().map(|&c| self.point(c)).collect();
        let lines =
            (0..points.len()).map(|i| self.line(points[i], points[(i + 1) % points.len()])).collect();
        (points, lines)
    }

    /// The rectangle from corner `a` to the opposite corner `b`, its sides parallel to the axes.
    pub fn rectangle(&mut self, a: [f64; 2], b: [f64; 2]) -> Vec<CurveId> {
        self.polygon(&[a, [b[0], a[1]], b, [a[0], b[1]]]).1
    }

    /// Marks `curve` as construction geometry: drawn to refer to, no part of the profile.
    pub fn construction(&mut self, curve: CurveId) {
        self.sketch.set_construction(curve, true);
    }
}

/// Adds the sketch `id`, `drawing` on `plane`.
pub fn sketch(b: &mut Bench, id: &str, plane: SketchPlane, drawing: Drawing) -> Result<(), Refusal> {
    b.step(id, AddSketchArgs::on(plane, drawing.sketch))
}

/// Extrudes the whole sketch `sketch` `length` along its plane's normal (backwards if negative),
/// as the step `id`, kept or combined as `combine` says.
pub fn extrude(
    b: &mut Bench,
    id: &str,
    sketch: &str,
    length: f64,
    combine: Combine,
) -> Result<(), Refusal> {
    b.step(
        id,
        ExtrudeArgs {
            sketch: whole_sketch(sketch),
            extent: Extents::blind(length),
            output: Output::Solid(combine),
            draft: 0.0,
        },
    )
}

/// Extrudes the whole sketch `sketch` `length` in all, half of it to each side of its plane, as
/// the step `id`, kept or combined as `combine` says.
pub fn extrude_symmetric(
    b: &mut Bench,
    id: &str,
    sketch: &str,
    length: f64,
    combine: Combine,
) -> Result<(), Refusal> {
    b.step(
        id,
        ExtrudeArgs {
            sketch: whole_sketch(sketch),
            extent: Extents { symmetric: true, ..Extents::blind(length) },
            output: Output::Solid(combine),
            draft: 0.0,
        },
    )
}

/// Revolves the whole sketch `sketch` `degrees` about `axis`, as the step `id`, kept or combined
/// as `combine` says.
pub fn revolve(
    b: &mut Bench,
    id: &str,
    sketch: &str,
    axis: RevolveAxis,
    degrees: f64,
    combine: Combine,
) -> Result<(), Refusal> {
    b.step(
        id,
        RevolveArgs {
            sketch: whole_sketch(sketch),
            axis: Some(axis),
            extent: Extents::blind(degrees),
            output: Output::Solid(combine),
        },
    )
}

/// The line `line` of the sketch `sketch`, as an axis to revolve about.
pub fn sketch_axis(sketch: &str, line: CurveId) -> RevolveAxis {
    RevolveAxis::Sketch(sketch_element(sketch, line))
}

/// The name of the solid the step `id` made.
pub fn made(kind: &str, id: &str) -> String {
    format!("{kind}({id})")
}
pub mod blends;
pub mod paths;
pub mod hollow;
