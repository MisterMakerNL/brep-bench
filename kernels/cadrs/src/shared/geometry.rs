//! What the cadrs implementations of the geometry cases share: ellipses in sketches; closed
//! sections drawn with curves a sketch cannot hold (conics, quadratic Béziers), filled and swept;
//! B-spline surfaces from their control nets, put in place of a face; the saddle drum.
//!
//! A cadrs sketch holds lines, arcs, circles, ellipses and cubic Béziers, but no parabola,
//! hyperbola or rational curve. cadrs draws those as 3D curves instead (`PathCurve::Bezier3`, a
//! Bézier with weights): the section is a wire of them (`Kernel::create_wire`), filled with its
//! plane (`Kernel::fill`, Onshape's Fill), and the flat sheet swept one kind higher into a solid
//! (`Kernel::sweep_body`: turned about an axis, or slid along a vector).
use crate::bench::{p2, p3, P2, P3, V3};
use crate::Refusal;
use cadrs_kernel::native::{NurbsSurface, Surface};
use cadrs_kernel::{
    Axis, BodyId, BodySweep, Curve2, DefaultKernel, FaceReplacement, FillCurve, FillSpec, Kernel,
    Loop, PathCurve, Plane,
};
use std::sync::Arc;

/// The whole ellipse about `center` with semi-axes `a` along the sketch's x and `b` along its y.
pub fn ellipse(center: (f64, f64), a: f64, b: f64) -> Loop {
    Loop {
        curves: vec![Curve2::Ellipse {
            center: p2(center.0, center.1),
            major_radius: a,
            minor_radius: b,
            rotation: 0.0,
            source: None,
        }],
    }
}

/// The arc of the ellipse about `center` with semi-axes `a` along the sketch's x and `b` along
/// its y, from parameter `start` degrees through `sweep` degrees.
pub fn ellipse_arc(center: (f64, f64), a: f64, b: f64, start: f64, sweep: f64) -> Curve2 {
    Curve2::EllipseArc {
        center: p2(center.0, center.1),
        major_radius: a,
        minor_radius: b,
        rotation: 0.0,
        start: start.to_radians(),
        sweep: sweep.to_radians(),
        source: None,
    }
}

/// A closed section on a plane, its curves in the plane's coordinates, one after another.
pub struct Section {
    plane: Plane,
    curves: Vec<PathCurve>,
}

impl Section {
    pub fn on(plane: Plane) -> Self {
        Section { plane, curves: Vec::new() }
    }

    /// The point of the plane at sketch coordinates `(x, y)`.
    fn at(&self, (x, y): (f64, f64)) -> P3 {
        let pl = &self.plane;
        let ydir = pl.normal.cross(&pl.x_dir);
        pl.origin + pl.x_dir.into_inner() * x + ydir * y
    }

    /// The segment from `a` to `b`: a sketch line.
    pub fn line(mut self, a: (f64, f64), b: (f64, f64)) -> Self {
        let curve = Curve2::Line { a: P2::new(a.0, a.1), b: P2::new(b.0, b.1), source: None };
        self.curves.push(PathCurve::Sketch { plane: self.plane, curve });
        self
    }

    /// The Bézier curve with these control points and weights (empty: a polynomial curve).
    pub fn bezier(mut self, poles: &[(f64, f64)], weights: &[f64]) -> Self {
        let poles = poles.iter().map(|&q| self.at(q)).collect();
        self.curves.push(PathCurve::Bezier3 { poles, weights: weights.to_vec() });
        self
    }

    /// The conic arc from `a` to `b` whose end tangents meet at `m`, weighted `w` there (a
    /// rational quadratic: a parabola when `w` = 1, a hyperbola above 1, an ellipse below).
    pub fn conic(self, a: (f64, f64), m: (f64, f64), w: f64, b: (f64, f64)) -> Self {
        self.bezier(&[a, m, b], &[1.0, w, 1.0])
    }

    /// The flat sheet the section bounds: its curves made a wire, filled with their plane.
    pub fn sheet(&self, k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
        let wire = first(k.create_wire(&self.curves)?)?;
        let curves = k
            .edges(wire)?
            .into_iter()
            .map(|e| FillCurve::Edge { body: wire, edge: e.id })
            .collect();
        first(k.fill(&FillSpec { curves, source: 1 })?)
    }

    /// The solid the section's sheet sweeps turned a whole turn about the axis through `origin`
    /// along `dir`.
    pub fn turned(&self, k: &mut DefaultKernel, origin: P3, dir: V3) -> Result<BodyId, Refusal> {
        let sheet = self.sheet(k)?;
        let axis = Axis { origin, dir: crate::bench::unit(dir) };
        first(k.sweep_body(sheet, &BodySweep::Rotate { axis, angle: std::f64::consts::TAU })?)
    }

    /// The solid the section's sheet sweeps slid along `v`.
    pub fn slid(&self, k: &mut DefaultKernel, v: V3) -> Result<BodyId, Refusal> {
        let sheet = self.sheet(k)?;
        first(k.sweep_body(sheet, &BodySweep::Translate(v))?)
    }
}

/// The section turned a whole turn about the z axis.
pub fn turned_about_z(k: &mut DefaultKernel, section: &Section) -> Result<BodyId, Refusal> {
    section.turned(k, p3(0.0, 0.0, 0.0), V3::z())
}

fn first(r: cadrs_kernel::OpResult) -> Result<BodyId, Refusal> {
    r.bodies.first().copied().ok_or_else(|| Refusal::from("the operation made no body"))
}

/// A control point `(x, y, z)` of weight `w`, homogeneous as cadrs's B-splines hold them.
pub fn pole(x: f64, y: f64, z: f64, w: f64) -> nalgebra::Vector4<f64> {
    nalgebra::Vector4::new(w * x, w * y, w * z, w)
}

/// The clamped knots of a Bézier of degree `p` on [0, 1].
pub fn bezier_knots(p: usize) -> Vec<f64> {
    let mut knots = vec![0.0; p + 1];
    knots.extend(vec![1.0; p + 1]);
    knots
}

/// The saddle z = 10 + xy/8 of `geometry/saddle-drum` as a B-spline surface: the bilinear patch
/// (degree 1 each way, non-rational) on the corners of the square −10 ≤ x, y ≤ 10 at the
/// saddle's heights there.
pub fn saddle() -> Result<Surface, Refusal> {
    let s = NurbsSurface::new(
        1,
        1,
        bezier_knots(1),
        bezier_knots(1),
        vec![
            vec![pole(-10.0, -10.0, 22.5, 1.0), pole(-10.0, 10.0, -2.5, 1.0)],
            vec![pole(10.0, -10.0, -2.5, 1.0), pole(10.0, 10.0, 22.5, 1.0)],
        ],
    )?;
    Ok(Surface::Nurbs(Arc::new(s)))
}

/// `body` with its face through `point` given the surface `surface` (cadrs's Replace face,
/// SMLib's local operation): the edges round it recomputed where the new surface meets the
/// faces next to it.
pub fn replace_face(
    k: &mut DefaultKernel,
    body: BodyId,
    point: P3,
    surface: Surface,
) -> Result<BodyId, Refusal> {
    let face = crate::bench::face_at(k, body, point)?;
    first(k.replace_faces(body, &[FaceReplacement { face, surface, domain: None }])?)
}

/// The drum of `geometry/saddle-drum`: a circle of radius 8 about the z axis on the Top plane,
/// extruded 20 (past the saddle's highest point over it, 14), its top face replaced by the
/// saddle.
pub fn saddle_drum(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let drum = crate::bench::cylinder(k, p3(0.0, 0.0, 0.0), V3::z(), 8.0, 20.0)?;
    replace_face(k, drum, p3(0.0, 0.0, 20.0), saddle()?)
}
