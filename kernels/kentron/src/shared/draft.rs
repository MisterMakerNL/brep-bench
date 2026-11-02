//! What the draft implementations share: faces of a solid drafted with kentron's `draft`, a box
//! with its vertical edges rounded, and a box's sides picked.
//!
//! `draft` turns each face it is given about its neutral curve, where the face crosses the
//! neutral plane, until it makes the given angle with the pull direction: a plane turns about a
//! line, and a cylinder about the pull direction becomes a cone through the same circle. A
//! positive angle leans the face toward the solid's inside as it runs along the pull direction
//! (an outer side narrows, a hole's wall widens), a negative one away from it.
use crate::bench::{edges, face_through, p};
use crate::Refusal;
use kentron::blend;
use kentron::draft::{self, DraftedFace};
use kentron::geometry::surface::Plane;
use kentron::geometry::ParametricCurve;
use kentron::math::{Direction, Point};
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

/// `solid` with each of `faces` drafted `degrees` about the plane through `neutral` square to
/// `pull`.
pub fn drafted(m: &mut Model, solid: Shape, faces: &[Shape], degrees: f64, neutral: Point, pull: Direction) -> Result<Shape, Refusal> {
    let plane = Plane::new(neutral, pull);
    let faces: Vec<DraftedFace> =
        faces.iter().map(|&face| DraftedFace { face, angle: degrees.to_radians(), neutral: plane }).collect();
    Ok(draft::draft(m, solid, pull, &faces)?)
}

/// The box from `lo` to `hi` with its four vertical edges, those whose ends differ only in z,
/// rounded to `radius`.
pub fn rounded_box(m: &mut Model, lo: Point, hi: Point, radius: f64) -> Result<Shape, Refusal> {
    let block = Cuboid::from_corners(lo, hi).build(m)?;
    let mut vertical = Vec::new();
    for edge in edges(m, block) {
        let (curve, first, last) = m.edge_curve(edge).ok_or("an edge of the box has no curve")?;
        let (a, b) = (curve.point(first), curve.point(last));
        if (a.x - b.x).abs() < 1e-9 && (a.y - b.y).abs() < 1e-9 {
            vertical.push(edge);
        }
    }
    Ok(blend::fillet(m, block, &vertical, radius)?)
}

/// The faces of `solid` where the box from `lo` to `hi` has its four sides, picked halfway up
/// each: its planes at their middles and, where its vertical edges are rounded to `radius`, the
/// four rounds at theirs. A pocket's walls are picked by its box the same way.
pub fn box_sides(m: &mut Model, solid: Shape, lo: Point, hi: Point, radius: f64) -> Result<Vec<Shape>, Refusal> {
    let (xm, ym, z) = ((lo.x + hi.x) / 2.0, (lo.y + hi.y) / 2.0, (lo.z + hi.z) / 2.0);
    let mut points = vec![p(lo.x, ym, z), p(hi.x, ym, z), p(xm, lo.y, z), p(xm, hi.y, z)];
    if radius > 0.0 {
        // A quarter round's middle, in from the box's corner.
        let k = radius * (1.0 - 0.5f64.sqrt());
        points.extend([
            p(lo.x + k, lo.y + k, z),
            p(hi.x - k, lo.y + k, z),
            p(hi.x - k, hi.y - k, z),
            p(lo.x + k, hi.y - k, z),
        ]);
    }
    points.into_iter().map(|q| face_through(m, solid, q)).collect()
}
