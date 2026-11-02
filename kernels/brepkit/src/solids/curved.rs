//! Helpers for the geometry, nurbs and spline cases: NURBS curves and surfaces from the control
//! points, weights and knots a case gives, through brepkit's own types, and loops of edges on them
//! made into faces.
use brepkit_math::curves::{Circle3D, Ellipse3D};
use brepkit_math::nurbs::curve::NurbsCurve;
use brepkit_math::nurbs::surface::NurbsSurface;
use brepkit_math::vec::{Point3, Vec3};
use brepkit_math::MathError;
use brepkit_operations::sew::sew_faces;
use brepkit_topology::builder::make_planar_face_from_wire;
use brepkit_topology::edge::{Edge, EdgeCurve};
use brepkit_topology::face::{Face, FaceId, FaceSurface};
use brepkit_topology::solid::SolidId;
use brepkit_topology::vertex::{Vertex, VertexId};
use brepkit_topology::wire::{OrientedEdge, Wire, WireId};
use brepkit_topology::Topology;

use crate::Refusal;

/// The tolerance vertices are made with.
pub const TOL: f64 = 1e-7;

/// A NURBS curve of `degree` on `knots`, its control points `points` with `weights`.
pub fn nurbs(
    degree: usize,
    knots: &[f64],
    points: &[Point3],
    weights: &[f64],
) -> Result<NurbsCurve, MathError> {
    NurbsCurve::new(degree, knots.to_vec(), points.to_vec(), weights.to_vec())
}

/// The non-rational Bézier curve on `points`, on the parameter 0 to 1.
pub fn bezier(points: &[Point3]) -> Result<NurbsCurve, MathError> {
    rational_bezier(points, &vec![1.0; points.len()])
}

/// The rational Bézier curve on `points` with `weights`, on the parameter 0 to 1.
pub fn rational_bezier(points: &[Point3], weights: &[f64]) -> Result<NurbsCurve, MathError> {
    let n = points.len();
    let knots: Vec<f64> = (0..2 * n).map(|i| if i < n { 0.0 } else { 1.0 }).collect();
    nurbs(n - 1, &knots, points, weights)
}

/// The uniform periodic B-spline of `degree` on the closed control polygon `points` (the last
/// joining back to the first). brepkit's NURBS has no periodic flag: the curve is the same
/// B-spline written unclamped, on evenly spaced knots, its first `degree` control points
/// repeated at its end, so that it closes on itself with no end anywhere.
pub fn periodic(degree: usize, points: &[Point3]) -> Result<NurbsCurve, MathError> {
    let mut wrapped = points.to_vec();
    wrapped.extend_from_slice(&points[..degree]);
    let knots: Vec<f64> = (0..wrapped.len() + degree + 1).map(|i| i as f64).collect();
    nurbs(degree, &knots, &wrapped, &vec![1.0; wrapped.len()])
}

/// The tensor-product NURBS surface of `degree_u` × `degree_v` on `knots_u` × `knots_v`, its
/// control points `points[i][j]` (row i along u) with `weights[i][j]`.
pub fn surface(
    degree_u: usize,
    degree_v: usize,
    knots_u: &[f64],
    knots_v: &[f64],
    points: Vec<Vec<Point3>>,
    weights: Vec<Vec<f64>>,
) -> Result<NurbsSurface, MathError> {
    NurbsSurface::new(
        degree_u,
        degree_v,
        knots_u.to_vec(),
        knots_v.to_vec(),
        points,
        weights,
    )
}

/// One piece of a loop of edges, run from its start to its end.
pub enum Seg {
    /// The straight line from the first point to the second.
    Line(Point3, Point3),
    /// A NURBS curve, from its start to its end.
    Curve(NurbsCurve),
    /// A NURBS curve run backwards, from its end to its start.
    Back(NurbsCurve),
    /// The part of a circle or ellipse from the first point to the second, anticlockwise about
    /// its normal.
    Conic(EdgeCurve, Point3, Point3),
}

impl Seg {
    /// The arc about `center` in the plane square to `normal`, from `from` to `to`,
    /// anticlockwise about `normal`.
    pub fn arc(center: Point3, normal: Vec3, from: Point3, to: Point3) -> Result<Seg, MathError> {
        let circle =
            Circle3D::new_with_ref(center, normal, (from - center).length(), from - center)?;
        Ok(Seg::Conic(EdgeCurve::Circle(circle), from, to))
    }

    /// The arc about `center` of the ellipse with semi-axes `major` along `major_dir` and `minor`
    /// across it, in the plane square to `normal`, from `from` to `to`, anticlockwise about
    /// `normal`.
    pub fn ellipse_arc(
        center: Point3,
        normal: Vec3,
        (major, minor): (f64, f64),
        major_dir: Vec3,
        from: Point3,
        to: Point3,
    ) -> Result<Seg, MathError> {
        let ellipse = Ellipse3D::new_with_ref(center, normal, major, minor, major_dir)?;
        Ok(Seg::Conic(EdgeCurve::Ellipse(ellipse), from, to))
    }

    /// Where it starts and ends.
    pub fn ends(&self) -> (Point3, Point3) {
        match self {
            Seg::Line(a, b) | Seg::Conic(_, a, b) => (*a, *b),
            Seg::Curve(c) | Seg::Back(c) => {
                let (t0, t1) = c.domain();
                let (a, b) = (c.evaluate(t0), c.evaluate(t1));
                if matches!(self, Seg::Curve(_)) {
                    (a, b)
                } else {
                    (b, a)
                }
            }
        }
    }
}

/// The vertex at `at` among `made`, or a new one there.
fn vertex_at(topo: &mut Topology, made: &mut Vec<(Point3, VertexId)>, at: Point3) -> VertexId {
    if let Some(&(_, v)) = made.iter().find(|(q, _)| (*q - at).length() <= 1e-6) {
        return v;
    }
    let v = topo.add_vertex(Vertex::new(at, TOL));
    made.push((at, v));
    v
}

/// The closed wire of `segs`, each running on from where the one before it ends, the last back
/// to where the first starts; pieces meeting at a point share its vertex.
pub fn wire(topo: &mut Topology, segs: Vec<Seg>) -> Result<WireId, Refusal> {
    let mut made = Vec::new();
    let mut edges = Vec::new();
    for seg in segs {
        let (a, b) = seg.ends();
        let (va, vb) = (vertex_at(topo, &mut made, a), vertex_at(topo, &mut made, b));
        let (edge, forward) = match seg {
            Seg::Line(..) => (Edge::new(va, vb, EdgeCurve::Line), true),
            Seg::Curve(c) => (Edge::new(va, vb, EdgeCurve::NurbsCurve(c)), true),
            Seg::Back(c) => (Edge::new(vb, va, EdgeCurve::NurbsCurve(c)), false),
            Seg::Conic(c, ..) => (Edge::new(va, vb, c), true),
        };
        edges.push(OrientedEdge::new(topo.add_edge(edge), forward));
    }
    Ok(topo.add_wire(Wire::new(edges, true)?))
}

/// The flat face bounded by `segs`.
pub fn plane_face(topo: &mut Topology, segs: Vec<Seg>) -> Result<FaceId, Refusal> {
    let w = wire(topo, segs)?;
    Ok(make_planar_face_from_wire(topo, w)?)
}

/// The face of `surface` bounded by `segs`, which lie on it.
pub fn surface_face(
    topo: &mut Topology,
    surface: NurbsSurface,
    segs: Vec<Seg>,
) -> Result<FaceId, Refusal> {
    let w = wire(topo, segs)?;
    Ok(topo.add_face(Face::new(w, vec![], FaceSurface::Nurbs(surface))))
}

/// The solid `faces` close, sewn together along their common edges.
pub fn sew(topo: &mut Topology, faces: &[FaceId]) -> Result<SolidId, Refusal> {
    Ok(sew_faces(topo, faces, 1e-6)?)
}

/// The block under the surface `top`, standing on the floor z = 0 under its four edges: its top
/// is the face of `top` bounded by those edges, each side the flat face between the floor and
/// one of them, all sewn together. The edges are the curves `u0`, `u1` (the surface's edges at
/// the start and end of u, each running along v) and `v0`, `v1` (at the start and end of v, each
/// running along u), which the caller makes from the surface's own boundary control points.
pub fn block_under(
    topo: &mut Topology,
    top: NurbsSurface,
    [u0, u1, v0, v1]: [NurbsCurve; 4],
) -> Result<SolidId, Refusal> {
    let floor = |q: Point3| Point3::new(q.x(), q.y(), 0.0);
    // the corners S(u, v) at the ends of the domain
    let ((c00, c01), (c10, c11)) = (Seg::Curve(u0.clone()).ends(), Seg::Curve(u1.clone()).ends());
    let mut faces = vec![surface_face(
        topo,
        top,
        vec![
            Seg::Curve(v0.clone()),
            Seg::Curve(u1.clone()),
            Seg::Back(v1.clone()),
            Seg::Back(u0.clone()),
        ],
    )?];
    for (edge, a, b) in [
        (v0, c00, c10),
        (u1, c10, c11),
        (v1, c01, c11),
        (u0, c00, c01),
    ] {
        faces.push(plane_face(
            topo,
            vec![
                Seg::Line(floor(a), floor(b)),
                Seg::Line(floor(b), b),
                Seg::Back(edge),
                Seg::Line(a, floor(a)),
            ],
        )?);
    }
    faces.push(plane_face(
        topo,
        vec![
            Seg::Line(floor(c00), floor(c01)),
            Seg::Line(floor(c01), floor(c11)),
            Seg::Line(floor(c11), floor(c10)),
            Seg::Line(floor(c10), floor(c00)),
        ],
    )?);
    sew(topo, &faces)
}

/// The block under the non-rational surface on the control points `points[i][j]` (row i along
/// u), of `degree_u` × `degree_v` on `knots_u` × `knots_v`. Its four edges are the curves on the
/// surface's boundary rows and columns of control points, on the same knots.
pub fn block_under_spline(
    topo: &mut Topology,
    (degree_u, degree_v): (usize, usize),
    knots_u: &[f64],
    knots_v: &[f64],
    points: Vec<Vec<Point3>>,
) -> Result<SolidId, Refusal> {
    let (rows, cols) = (points.len(), points[0].len());
    let column = |j: usize| points.iter().map(|row| row[j]).collect::<Vec<_>>();
    let u0 = nurbs(degree_v, knots_v, &points[0], &vec![1.0; cols])?;
    let u1 = nurbs(degree_v, knots_v, &points[rows - 1], &vec![1.0; cols])?;
    let v0 = nurbs(degree_u, knots_u, &column(0), &vec![1.0; rows])?;
    let v1 = nurbs(degree_u, knots_u, &column(cols - 1), &vec![1.0; rows])?;
    let top = surface(
        degree_u,
        degree_v,
        knots_u,
        knots_v,
        points,
        vec![vec![1.0; cols]; rows],
    )?;
    block_under(topo, top, [u0, u1, v0, v1])
}

/// The knots of a Bézier of `degree`: `degree + 1` zeros and as many ones.
pub fn bezier_knots(degree: usize) -> Vec<f64> {
    (0..2 * degree + 2)
        .map(|i| if i <= degree { 0.0 } else { 1.0 })
        .collect()
}
