//! Helpers shared by the offset and thicken cases: picking a solid's faces by the kind of surface
//! they lie on, thickening several faces of a solid as one sheet with brepkit's shells, and the
//! solids and sheets several cases share.
use brepkit_math::curves::Circle3D;
use brepkit_math::surfaces::SphericalSurface;
use brepkit_math::vec::{Point3, Vec3};
use brepkit_operations::extrude::extrude;
use brepkit_operations::offset_v2::shell_v2;
use brepkit_operations::primitives::{make_box, make_cylinder};
use brepkit_operations::shell_op::shell;
use brepkit_operations::OperationsError;
use brepkit_topology::builder::{
    make_circle_edge, make_face_from_wire, make_line_edge, make_planar_face,
};
use brepkit_topology::edge::{Edge, EdgeCurve, EdgeId};
use brepkit_topology::explorer::solid_faces;
use brepkit_topology::face::{Face, FaceId, FaceSurface};
use brepkit_topology::solid::SolidId;
use brepkit_topology::vertex::Vertex;
use brepkit_topology::wire::{OrientedEdge, Wire};
use brepkit_topology::Topology;

use crate::solids::{common, moved, p, v};
use crate::Refusal;

/// The faces of `solid` whose surface `on` accepts.
pub fn faces_on(
    topo: &Topology,
    solid: SolidId,
    on: impl Fn(&FaceSurface) -> bool,
) -> Result<Vec<FaceId>, OperationsError> {
    let mut out = Vec::new();
    for f in solid_faces(topo, solid)? {
        if on(topo.face(f)?.surface()) {
            out.push(f);
        }
    }
    Ok(out)
}

/// The one face of `solid` whose surface `on` accepts.
pub fn face_on(
    topo: &Topology,
    solid: SolidId,
    on: impl Fn(&FaceSurface) -> bool,
) -> Result<FaceId, Refusal> {
    match faces_on(topo, solid, on)?[..] {
        [f] => Ok(f),
        ref fs => Err(format!("expected one such face, found {}", fs.len()).into()),
    }
}

/// The sheet of `sheet`'s faces of `solid` thickened by `thickness` with brepkit's shells, every
/// other face of `solid` an opening: inside the faces by brepkit's `shell` (the one brepjs calls)
/// when `thickness` is negative, and outside them by `shell_v2`, the only one that grows a wall
/// outward, when positive. Both extend the moved faces until they meet.
pub fn thicken_faces(
    topo: &mut Topology,
    solid: SolidId,
    sheet: &[FaceId],
    thickness: f64,
) -> Result<SolidId, OperationsError> {
    let openings: Vec<FaceId> = solid_faces(topo, solid)?
        .into_iter()
        .filter(|f| !sheet.contains(f))
        .collect();
    if thickness < 0.0 {
        shell(topo, solid, -thickness, &openings)
    } else {
        shell_v2(topo, solid, thickness, &openings)
    }
}

/// The L-shaped plate of the offset cases: the outline through (0, 0), (20, 0), (20, 5), (5, 5),
/// (5, 15) and (0, 15) in the xy plane, extruded 8 along +z.
pub fn l_block(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    let outline = [
        (0.0, 0.0),
        (20.0, 0.0),
        (20.0, 5.0),
        (5.0, 5.0),
        (5.0, 15.0),
        (0.0, 15.0),
    ]
    .map(|(x, y)| p(x, y, 0.0));
    let face = make_planar_face(topo, &outline, 1e-7)?;
    extrude(topo, face, v(0.0, 0.0, 1.0), 8.0)
}

/// The arc of the circle about `center` in the plane normal to `axis`, from `start` round
/// counter-clockwise (seen down `axis`) to `end`, as brepkit's own `makeCircleArc3d` makes it.
pub fn arc(
    topo: &mut Topology,
    start: Point3,
    end: Point3,
    center: Point3,
    axis: Vec3,
) -> Result<EdgeId, Refusal> {
    let n = axis.normalize()?;
    let radial = start - center;
    let radius = radial.length();
    let u = radial * (1.0 / radius);
    let circle = Circle3D::with_axes(center, n, radius, u, n.cross(u))?;
    let a = topo.add_vertex(Vertex::new(start, 1e-7));
    let b = topo.add_vertex(Vertex::new(end, 1e-7));
    Ok(topo.add_edge(Edge::new(a, b, EdgeCurve::Circle(circle))))
}

/// The half of brepkit's cylinder of `radius` on the z axis from z = 0 to `height` where y ≥ 0:
/// the cylinder's common part with a box that runs past it everywhere but at y = 0.
pub fn half_cylinder(
    topo: &mut Topology,
    radius: f64,
    height: f64,
) -> Result<SolidId, OperationsError> {
    let cylinder = make_cylinder(topo, radius, height)?;
    let half = make_box(topo, 2.0 * radius + 2.0, radius + 1.0, height + 2.0)?;
    let half = moved(topo, half, -radius - 1.0, 0.0, -1.0)?;
    common(topo, cylinder, half)
}

/// The cap of the sphere of `radius` about the origin above the plane z = `z`, as one face on the
/// sphere bounded by the circle where that plane cuts it, its normal pointing out of the sphere:
/// the circle runs counter-clockwise seen from +z, as round the north face of brepkit's own ball.
pub fn sphere_cap(topo: &mut Topology, radius: f64, z: f64) -> Result<FaceId, Refusal> {
    let rim_radius = (radius * radius - z * z).sqrt();
    let rim = make_circle_edge(topo, p(0.0, 0.0, z), v(0.0, 0.0, 1.0), rim_radius, 1e-7)?;
    let rim = topo.add_wire(Wire::new(vec![OrientedEdge::new(rim, true)], true)?);
    let sphere = SphericalSurface::new(p(0.0, 0.0, 0.0), radius)?;
    Ok(topo.add_face(Face::new(rim, vec![], FaceSurface::Sphere(sphere))))
}

/// The solid of the bent sheet of `thicken/bent-flange`: its profile in the xz plane (the floor
/// from (−10, 0, 0) to the origin, the quarter circle of radius 2 about (0, 0, 2) to (2, 0, 2),
/// the wall up to (2, 0, 12)) closed across the top to (−10, 0, 12) and down to its start, and
/// extruded 8 along +y. Its floor, bend and wall are the sheet's three faces.
pub fn bent_block(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let bend = arc(
        topo,
        p(0.0, 0.0, 0.0),
        p(2.0, 0.0, 2.0),
        p(0.0, 0.0, 2.0),
        v(0.0, -1.0, 0.0),
    )?;
    let mut edges = vec![
        make_line_edge(topo, p(-10.0, 0.0, 0.0), p(0.0, 0.0, 0.0), 1e-7)?,
        bend,
    ];
    let corners = [
        p(2.0, 0.0, 2.0),
        p(2.0, 0.0, 12.0),
        p(-10.0, 0.0, 12.0),
        p(-10.0, 0.0, 0.0),
    ];
    for w in corners.windows(2) {
        edges.push(make_line_edge(topo, w[0], w[1], 1e-7)?);
    }
    let profile = Wire::new(
        edges
            .into_iter()
            .map(|e| OrientedEdge::new(e, true))
            .collect(),
        true,
    )?;
    let profile = topo.add_wire(profile);
    let face = make_face_from_wire(topo, profile)?;
    Ok(extrude(topo, face, v(0.0, 1.0, 0.0), 8.0)?)
}

/// The floor, bend and wall of [`bent_block`]: its faces on the plane z = 0, the cylinder, and
/// the plane x = 2.
pub fn bent_sheet(topo: &Topology, block: SolidId) -> Result<Vec<FaceId>, OperationsError> {
    faces_on(topo, block, |s| match s {
        FaceSurface::Cylinder(_) => true,
        FaceSurface::Plane { normal, d } => {
            (normal.z().abs() > 0.5 && d.abs() < 1e-7)
                || (normal.x().abs() > 0.5 && (d.abs() - 2.0).abs() < 1e-7)
        }
        _ => false,
    })
}
