//! A square spire on four Bézier ridges. brepkit's loft runs through sections (and to a point
//! only straight), and its guided sweep only turns a section of fixed shape toward one guide, so
//! it has no loft along guides. The solid is built from what its walls are: the section's sides
//! stay parallel to x and y, so the walls facing ±x are the ridge seen along y, (g, z), and its
//! mirror image in the yz plane, slid along y, and those facing ±y are the same seen along x,
//! slid along x. Each pair bounds a region of its plane, closed by the floor and meeting at the
//! apex, which is extruded through the solid; the solid is the common of the two extrusions.
use brepkit_math::vec::Point3;
use brepkit_operations::extrude::extrude;
use brepkit_topology::edge::EdgeCurve;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::sweep::{bezier, chain};
use crate::solids::{common, moved, p, v};
use crate::Refusal;

/// The ridge through the corner with x and y positive, seen along x or y, (g, z): its control
/// points.
const RIDGE: [(f64, f64); 4] = [(9.0, 0.0), (15.0, 8.0), (12.0, 19.0), (0.0, 24.0)];

/// The region between the ridge and its mirror image across the z axis, in their plane (`at`
/// places the ridge's own coordinate and z): up the ridge to the apex, down its mirror image,
/// and back along the floor.
fn between(
    topo: &mut Topology,
    at: impl Fn(f64, f64) -> Point3,
) -> Result<brepkit_topology::face::FaceId, Refusal> {
    let up: Vec<Point3> = RIDGE.iter().map(|&(c, z)| at(c, z)).collect();
    let down: Vec<Point3> = RIDGE.iter().rev().map(|&(c, z)| at(-c, z)).collect();
    Ok(chain(
        topo,
        &[
            (up[0], EdgeCurve::NurbsCurve(bezier(&up)?)),
            (down[0], EdgeCurve::NurbsCurve(bezier(&down)?)),
            (down[3], EdgeCurve::Line),
        ],
    )?)
}

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    // each region extruded from 40 before its plane to 40 beyond
    let xz = between(topo, |c, z| p(c, 0.0, z))?;
    let walls_x = extrude(topo, xz, v(0.0, 1.0, 0.0), 80.0)?;
    let walls_x = moved(topo, walls_x, 0.0, -40.0, 0.0)?;
    let yz = between(topo, |c, z| p(0.0, c, z))?;
    let walls_y = extrude(topo, yz, v(1.0, 0.0, 0.0), 80.0)?;
    let walls_y = moved(topo, walls_y, -40.0, 0.0, 0.0)?;
    Ok(common(topo, walls_x, walls_y)?)
}
