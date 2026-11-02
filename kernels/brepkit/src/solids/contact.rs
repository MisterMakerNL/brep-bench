//! Helpers for the tangent, feature and parts cases: the primitives they share, placed where the
//! cases put them (each a brepkit primitive moved by a rigid motion), and a planar face bounded
//! by an outline of lines and arcs, made of brepkit's edges.
use brepkit_math::curves::Circle3D;
use brepkit_math::mat::Mat4;
use brepkit_math::vec::{Point3, Vec3};
use brepkit_operations::primitives::{make_box, make_cylinder, make_sphere};
use brepkit_operations::OperationsError;
use brepkit_topology::builder::make_planar_face_from_wire;
use brepkit_topology::edge::{Edge, EdgeCurve};
use brepkit_topology::face::FaceId;
use brepkit_topology::solid::SolidId;
use brepkit_topology::vertex::Vertex;
use brepkit_topology::wire::{OrientedEdge, Wire};
use brepkit_topology::Topology;
use std::f64::consts::FRAC_PI_2;

use super::{moved, placed};
use crate::Refusal;

/// The tolerance brepkit gives the vertices it makes.
const TOLERANCE: f64 = 1e-7;

/// How many vertices brepkit puts round a ball's equator; the face is an exact sphere whatever it
/// is, and it only steers the Booleans' sampling of curved intersections.
pub const SPHERE_SEGMENTS: usize = 64;

/// The block from `(x0, y0, z0)` to `(x1, y1, z1)`.
pub fn block(
    topo: &mut Topology,
    (x0, y0, z0): (f64, f64, f64),
    (x1, y1, z1): (f64, f64, f64),
) -> Result<SolidId, OperationsError> {
    let b = make_box(topo, x1 - x0, y1 - y0, z1 - z0)?;
    moved(topo, b, x0, y0, z0)
}

/// The ball of radius `r` about `(x, y, z)`.
pub fn ball(
    topo: &mut Topology,
    r: f64,
    (x, y, z): (f64, f64, f64),
) -> Result<SolidId, OperationsError> {
    let b = make_sphere(topo, r, SPHERE_SEGMENTS)?;
    moved(topo, b, x, y, z)
}

/// The cylinder of radius `r` on the vertical line through `(x, y)`, from `z0` to `z1`.
pub fn cylinder_z(
    topo: &mut Topology,
    r: f64,
    (x, y): (f64, f64),
    z0: f64,
    z1: f64,
) -> Result<SolidId, OperationsError> {
    let c = make_cylinder(topo, r, z1 - z0)?;
    moved(topo, c, x, y, z0)
}

/// The cylinder of radius `r` on the line parallel to x through `(y, z)`, from `x0` to `x1`:
/// brepkit's cylinder, on z, turned a quarter turn about y onto x.
pub fn cylinder_x(
    topo: &mut Topology,
    r: f64,
    (y, z): (f64, f64),
    x0: f64,
    x1: f64,
) -> Result<SolidId, OperationsError> {
    let c = make_cylinder(topo, r, x1 - x0)?;
    placed(
        topo,
        c,
        Mat4::translation(x0, y, z) * Mat4::rotation_y(FRAC_PI_2),
    )
}

/// The cylinder of radius `r` on the line parallel to y through `(x, z)`, from `y0` to `y1`:
/// brepkit's cylinder, on z, turned a quarter turn about x onto y.
pub fn cylinder_y(
    topo: &mut Topology,
    r: f64,
    (x, z): (f64, f64),
    y0: f64,
    y1: f64,
) -> Result<SolidId, OperationsError> {
    let c = make_cylinder(topo, r, y1 - y0)?;
    placed(
        topo,
        c,
        Mat4::translation(x, y0, z) * Mat4::rotation_x(-FRAC_PI_2),
    )
}

/// How an outline runs from one of its corners to the next.
pub enum Run {
    /// A straight line.
    Line,
    /// An arc of the circle about `centre` in the outline's plane, turning counter-clockwise about
    /// `axis` (the right-hand rule) from this corner to the next.
    Arc { centre: Point3, axis: Vec3 },
}

/// The planar face bounded by the closed outline through `corners`, each run to the next (the last
/// back to the first) as it says: brepkit's line and circle edges, sharing their vertices, in one
/// wire.
pub fn outline_face(topo: &mut Topology, corners: &[(Point3, Run)]) -> Result<FaceId, Refusal> {
    let vertices: Vec<_> = corners
        .iter()
        .map(|(at, _)| topo.add_vertex(Vertex::new(*at, TOLERANCE)))
        .collect();
    let mut edges = Vec::with_capacity(corners.len());
    for (i, (from, run)) in corners.iter().enumerate() {
        let next = (i + 1) % corners.len();
        let curve = match run {
            Run::Line => EdgeCurve::Line,
            Run::Arc { centre, axis } => {
                let radius = (*from - *centre).length();
                EdgeCurve::Circle(Circle3D::new_with_ref(
                    *centre,
                    *axis,
                    radius,
                    *from - *centre,
                )?)
            }
        };
        let edge = topo.add_edge(Edge::new(vertices[i], vertices[next], curve));
        edges.push(OrientedEdge::new(edge, true));
    }
    let wire = topo.add_wire(Wire::new(edges, true)?);
    Ok(make_planar_face_from_wire(topo, wire)?)
}
