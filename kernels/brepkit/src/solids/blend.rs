//! Helpers for the fillet, chamfer and draft cases: a blend that must take every edge it is given,
//! chamfers one edge after another, the solids several cases start from, and a draft with the
//! pull up +z.
use brepkit_math::vec::Point3;
use brepkit_operations::blend_ops::{all_blended, chamfer_v2, fillet_v2};
use brepkit_operations::chamfer::chamfer;
use brepkit_operations::draft::draft;
use brepkit_operations::extrude::extrude;
use brepkit_operations::primitives::make_convex_hull;
use brepkit_operations::OperationsError;
use brepkit_topology::builder::{make_face_from_wire, make_polygon_wire};
use brepkit_topology::edge::EdgeId;
use brepkit_topology::face::FaceId;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use super::{edges_where, faces_where, p, v};
use crate::Refusal;

/// `solid` with `edges` rounded to radius `r` in one fillet; an edge the fillet leaves out is an
/// error, not a partial result.
pub fn rounded(
    topo: &mut Topology,
    solid: SolidId,
    edges: &[EdgeId],
    r: f64,
) -> Result<SolidId, OperationsError> {
    all_blended(fillet_v2(topo, solid, edges, r)?)
}

/// `solid` with `edges` chamfered in one chamfer, the same distance `d` on both faces of each, as
/// brepkit's own front end (brepjs's `chamfer`, the WASM `try_chamfer`) does it: the blend
/// engine's chamfer first, which must take every edge, and where it declines, the planar chamfer
/// (`chamfer::chamfer`, which also cuts circular rims). If both decline, the error says why each
/// did.
pub fn chamfered(
    topo: &mut Topology,
    solid: SolidId,
    edges: &[EdgeId],
    d: f64,
) -> Result<SolidId, Refusal> {
    let blend = match chamfer_v2(topo, solid, edges, d, d).and_then(all_blended) {
        Ok(chamfered) => return Ok(chamfered),
        Err(why) => why,
    };
    chamfer(topo, solid, edges, d)
        .map_err(|planar| format!("blend chamfer: {blend}; planar chamfer: {planar}").into())
}

/// The octahedron |x| + |y| + |z| ≤ 10: the convex hull of its six corners on the axes.
pub fn octahedron(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    let corners = [
        p(10.0, 0.0, 0.0),
        p(-10.0, 0.0, 0.0),
        p(0.0, 10.0, 0.0),
        p(0.0, -10.0, 0.0),
        p(0.0, 0.0, 10.0),
        p(0.0, 0.0, -10.0),
    ];
    make_convex_hull(topo, &corners)
}

/// `solid` with each of `edges` chamfered in turn, each its own chamfer with distance `d` on both
/// faces, ending on the faces at its ends (the chamfers made before it among them). An edge is
/// found again in each new solid as the one left on its line.
pub fn chamfered_one_by_one(
    topo: &mut Topology,
    mut solid: SolidId,
    edges: &[EdgeId],
    d: f64,
) -> Result<SolidId, Refusal> {
    let mut lines = Vec::new();
    for &e in edges {
        let edge = topo.edge(e)?;
        let (a, b) = (
            topo.vertex(edge.start())?.point(),
            topo.vertex(edge.end())?.point(),
        );
        lines.push((a, (b - a).normalize()?));
    }
    for (a, along) in lines {
        let edge = edges_where(topo, solid, |q| (q - a).cross(along).length() < 1e-7)?;
        solid = chamfered(topo, solid, &edge, d)?;
    }
    Ok(solid)
}

/// The prism swept by the planar polygon through `corners` moved by `(dx, dy, dz)`.
pub fn prism(
    topo: &mut Topology,
    corners: &[Point3],
    (dx, dy, dz): (f64, f64, f64),
) -> Result<SolidId, OperationsError> {
    let wire = make_polygon_wire(topo, corners, 1e-7)?;
    let face = make_face_from_wire(topo, wire)?;
    let dir = v(dx, dy, dz);
    extrude(topo, face, dir.normalize()?, dir.length())
}

/// The L-block of `fillet/l-block-inside-consumed` and `chamfer/l-block-inside-consumed`: the
/// section through (0, 0), (10, 0), (10, 3), (3, 3), (3, 10), (0, 10) in xz, run along +y for 10.
pub fn l_block_xz(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    let section = [
        p(0.0, 0.0, 0.0),
        p(10.0, 0.0, 0.0),
        p(10.0, 0.0, 3.0),
        p(3.0, 0.0, 3.0),
        p(3.0, 0.0, 10.0),
        p(0.0, 0.0, 10.0),
    ];
    prism(topo, &section, (0.0, 10.0, 0.0))
}

/// The faces of `solid` lying wholly on one of `planes` (each a test of a point), as many as each
/// holds.
pub fn faces_on(
    topo: &Topology,
    solid: SolidId,
    planes: &[&dyn Fn(Point3) -> bool],
) -> Result<Vec<FaceId>, OperationsError> {
    let mut out = Vec::new();
    for on in planes {
        out.extend(faces_where(topo, solid, on)?);
    }
    Ok(out)
}

/// `solid` with `faces` drafted by `degrees`, the pull direction +z and the neutral plane
/// z = `neutral_z`; a positive angle leans each face in toward the material as it rises.
pub fn drafted_up(
    topo: &mut Topology,
    solid: SolidId,
    faces: &[FaceId],
    neutral_z: f64,
    degrees: f64,
) -> Result<SolidId, OperationsError> {
    draft(
        topo,
        solid,
        faces,
        v(0.0, 0.0, 1.0),
        p(0.0, 0.0, neutral_z),
        degrees.to_radians(),
    )
}
