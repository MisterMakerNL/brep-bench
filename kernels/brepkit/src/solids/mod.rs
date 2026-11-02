//! Small helpers shared by brepkit's implementations of the cases: points and vectors, placing a
//! solid by a rigid motion, the three Booleans, and picking a solid's edges and faces by where
//! they lie. Everything here only calls brepkit; a case file should still read as an example of
//! its use. Helpers that only one category needs live in that category's module below.
use brepkit_math::mat::Mat4;
use brepkit_math::vec::{Point3, Vec3};
use brepkit_operations::boolean::{boolean, BooleanOp};
use brepkit_operations::transform::transform_solid;
use brepkit_operations::OperationsError;
use brepkit_topology::edge::EdgeId;
use brepkit_topology::explorer::{solid_edges, solid_faces};
use brepkit_topology::face::FaceId;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

// one module per group of categories, for its own helpers
pub mod basic; // primitives, boolean
pub mod blend; // fillet, chamfer, draft
pub mod contact; // tangent, feature, parts
pub mod curved; // geometry, nurbs, spline
pub mod hollow; // shell
pub mod offset; // offset, thicken
pub mod sweep; // sweep, swept, loft

/// The point (x, y, z).
pub fn p(x: f64, y: f64, z: f64) -> Point3 {
    Point3::new(x, y, z)
}

/// The vector (x, y, z).
pub fn v(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3::new(x, y, z)
}

/// `solid` moved in place by `m`, and returned.
pub fn placed(topo: &mut Topology, solid: SolidId, m: Mat4) -> Result<SolidId, OperationsError> {
    transform_solid(topo, solid, &m)?;
    Ok(solid)
}

/// `solid` moved in place by (x, y, z), and returned.
pub fn moved(
    topo: &mut Topology,
    solid: SolidId,
    x: f64,
    y: f64,
    z: f64,
) -> Result<SolidId, OperationsError> {
    placed(topo, solid, Mat4::translation(x, y, z))
}

/// `a` less `b`.
pub fn cut(topo: &mut Topology, a: SolidId, b: SolidId) -> Result<SolidId, OperationsError> {
    boolean(topo, BooleanOp::Cut, a, b)
}

/// `a` and `b` together.
pub fn fuse(topo: &mut Topology, a: SolidId, b: SolidId) -> Result<SolidId, OperationsError> {
    boolean(topo, BooleanOp::Fuse, a, b)
}

/// What `a` and `b` share.
pub fn common(topo: &mut Topology, a: SolidId, b: SolidId) -> Result<SolidId, OperationsError> {
    boolean(topo, BooleanOp::Intersect, a, b)
}

/// Sample points along `edge`: its two ends and `n - 1` points between them, evenly spaced in
/// its curve's parameter.
pub fn edge_points(topo: &Topology, edge: EdgeId, n: usize) -> Result<Vec<Point3>, OperationsError> {
    let e = topo.edge(edge)?;
    let (a, b) = (topo.vertex(e.start())?.point(), topo.vertex(e.end())?.point());
    let (t0, t1) = e.curve().domain_with_endpoints(a, b);
    Ok((0..=n)
        .map(|i| e.curve().evaluate_with_endpoints(t0 + (t1 - t0) * i as f64 / n as f64, a, b))
        .collect())
}

/// The edges of `solid` every point of which (sampled) satisfies `on`.
pub fn edges_where(
    topo: &Topology,
    solid: SolidId,
    on: impl Fn(Point3) -> bool,
) -> Result<Vec<EdgeId>, OperationsError> {
    let mut out = Vec::new();
    for e in solid_edges(topo, solid)? {
        if edge_points(topo, e, 8)?.into_iter().all(&on) {
            out.push(e);
        }
    }
    Ok(out)
}

/// The faces of `solid` whose boundary's vertices all satisfy `on`.
pub fn faces_where(
    topo: &Topology,
    solid: SolidId,
    on: impl Fn(Point3) -> bool,
) -> Result<Vec<FaceId>, OperationsError> {
    let mut out = Vec::new();
    for f in solid_faces(topo, solid)? {
        let mut all = true;
        for e in brepkit_topology::explorer::face_edges(topo, f)? {
            all &= edge_points(topo, e, 8)?.into_iter().all(&on);
        }
        if all {
            out.push(f);
        }
    }
    Ok(out)
}

/// `x` is within 1e-7 of `y`.
pub fn near(x: f64, y: f64) -> bool {
    (x - y).abs() <= 1e-7
}
