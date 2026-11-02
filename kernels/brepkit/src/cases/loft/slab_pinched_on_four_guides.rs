//! A slab pinched thin on four Bézier guides. brepkit's loft runs through sections, and its
//! guided sweep only turns a section of fixed shape toward one guide, so it has no loft along
//! guides. The solid is built from what its walls are: the section's sides stay parallel to x and
//! y, so the walls facing ±x are the guide seen along y, (p, z), and its mirror image in the
//! yz plane, slid along y, and those facing ±y are the guide seen along x, (q, z), and its mirror
//! image, slid along x. Each pair bounds a region of its plane, which is extruded through the
//! solid, and the solid is the common of the two extrusions.
use brepkit_math::vec::Point3;
use brepkit_operations::extrude::extrude;
use brepkit_topology::edge::EdgeCurve;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::sweep::{bezier, chain};
use crate::solids::{common, moved, p, v};
use crate::Refusal;

/// The guide through the corner with x and y positive, (p, q, z): its control points.
const GUIDE: [(f64, f64, f64); 4] = [
    (12.0, 3.2, 0.0),
    (18.0, -2.8, 8.0),
    (15.0, 0.2, 19.0),
    (3.0, 12.2, 24.0),
];

/// The region between the Bézier on `points` and its mirror image across the z axis, in their
/// plane (`at` places the curve's own coordinate and z), from the foot to the top.
fn between(
    topo: &mut Topology,
    points: &[(f64, f64)],
    at: impl Fn(f64, f64) -> Point3,
) -> Result<brepkit_topology::face::FaceId, Refusal> {
    let up: Vec<Point3> = points.iter().map(|&(c, z)| at(c, z)).collect();
    // the mirror image, run from the top down
    let down: Vec<Point3> = points.iter().rev().map(|&(c, z)| at(-c, z)).collect();
    Ok(chain(
        topo,
        &[
            (up[0], EdgeCurve::NurbsCurve(bezier(&up)?)),
            (up[3], EdgeCurve::Line),
            (down[0], EdgeCurve::NurbsCurve(bezier(&down)?)),
            (down[3], EdgeCurve::Line),
        ],
    )?)
}

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let seen_along_y: Vec<_> = GUIDE.iter().map(|&(x, _, z)| (x, z)).collect();
    let seen_along_x: Vec<_> = GUIDE.iter().map(|&(_, y, z)| (y, z)).collect();
    // each region extruded from 40 before its plane to 40 beyond
    let xz = between(topo, &seen_along_y, |c, z| p(c, 0.0, z))?;
    let walls_x = extrude(topo, xz, v(0.0, 1.0, 0.0), 80.0)?;
    let walls_x = moved(topo, walls_x, 0.0, -40.0, 0.0)?;
    let yz = between(topo, &seen_along_x, |c, z| p(0.0, c, z))?;
    let walls_y = extrude(topo, yz, v(1.0, 0.0, 0.0), 80.0)?;
    let walls_y = moved(topo, walls_y, -40.0, 0.0, 0.0)?;
    Ok(common(topo, walls_x, walls_y)?)
}
