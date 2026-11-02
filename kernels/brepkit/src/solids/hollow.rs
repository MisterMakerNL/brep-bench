//! Helpers for the shell cases: picking the face an opening is through, and the solids several
//! cases shell (a polygonal plate, the drilled block, the rounded plate, the capsule's end, a ball
//! cut off by a plane).
use brepkit_math::vec::Point3;
use brepkit_operations::blend_ops::{all_blended, fillet_v2};
use brepkit_operations::distance::point_to_face;
use brepkit_operations::extrude::extrude;
use brepkit_operations::primitives::{make_box, make_cylinder, make_sphere};
use brepkit_operations::OperationsError;
use brepkit_topology::builder::{make_face_from_wire, make_polygon_wire};
use brepkit_topology::explorer::solid_faces;
use brepkit_topology::face::FaceId;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::{cut, edges_where, fuse, moved, near, p, v};
use crate::Refusal;

/// The face of `solid` that `point` lies on.
pub fn face_through(topo: &Topology, solid: SolidId, point: Point3) -> Result<FaceId, Refusal> {
    for f in solid_faces(topo, solid)? {
        if point_to_face(topo, point, f)?.distance <= 1e-7 {
            return Ok(f);
        }
    }
    Err(format!("no face of the solid passes through {point:?}").into())
}

/// The faces of `solid` that `points` lie on, one each.
pub fn faces_through(
    topo: &Topology,
    solid: SolidId,
    points: &[Point3],
) -> Result<Vec<FaceId>, Refusal> {
    points
        .iter()
        .map(|&q| face_through(topo, solid, q))
        .collect()
}

/// The plate whose outline in the xy plane is the polygon through `outline`, extruded `height`
/// along +z.
pub fn plate(
    topo: &mut Topology,
    outline: &[(f64, f64)],
    height: f64,
) -> Result<SolidId, OperationsError> {
    let corners: Vec<Point3> = outline.iter().map(|&(x, y)| p(x, y, 0.0)).collect();
    let wire = make_polygon_wire(topo, &corners, 1e-7)?;
    let face = make_face_from_wire(topo, wire)?;
    extrude(topo, face, v(0.0, 0.0, 1.0), height)
}

/// The block from the origin to (20, 20, 10) with a hole of radius 3 bored right through it
/// along z about the line through (`x`, `y`); the drill runs 1 past both faces.
pub fn drilled_block(topo: &mut Topology, x: f64, y: f64) -> Result<SolidId, OperationsError> {
    let block = make_box(topo, 20.0, 20.0, 10.0)?;
    let drill = make_cylinder(topo, 3.0, 12.0)?;
    let drill = moved(topo, drill, x, y, -1.0)?;
    cut(topo, block, drill)
}

/// The box from the origin to (20, 14, 10) with its four vertical edges rounded to radius 2.
pub fn rounded_plate(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let block = make_box(topo, 20.0, 14.0, 10.0)?;
    // the vertical edges: x and y fixed at a corner of the rectangle
    let corner =
        |a: f64, b: f64| (near(a, 0.0) || near(a, 20.0)) && (near(b, 0.0) || near(b, 14.0));
    let uprights = edges_where(topo, block, |q| corner(q.x(), q.y()))?;
    Ok(all_blended(fillet_v2(topo, block, &uprights, 2.0)?)?)
}

/// A capsule's end: the cylinder of radius 5 on the z axis from z = 0 to z = 10, fused with the
/// ball of radius 5 about (0, 0, 10), whose upper half is its dome.
pub fn capsule_end(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    let side = make_cylinder(topo, 5.0, 10.0)?;
    let ball = make_sphere(topo, 5.0, 64)?;
    let ball = moved(topo, ball, 0.0, 0.0, 10.0)?;
    fuse(topo, side, ball)
}

/// The part of the ball of radius 5 about the origin below the plane z = `h`: the ball less a box
/// standing on the plane.
pub fn ball_below(topo: &mut Topology, h: f64) -> Result<SolidId, OperationsError> {
    let ball = make_sphere(topo, 5.0, 64)?;
    let above = make_box(topo, 12.0, 12.0, 6.0)?;
    let above = moved(topo, above, -6.0, -6.0, h)?;
    cut(topo, ball, above)
}
