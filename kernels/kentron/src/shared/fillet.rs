//! What the fillet implementations share.
use kentron::make;
use kentron::math::{Point, Vector};
use kentron::sweep;
use kentron::topology::{Model, Shape};

/// The prism of the closed polygon through `points`, extruded along `v`.
pub fn prism(m: &mut Model, points: &[Point], v: Vector) -> kentron::Result<Shape> {
    let mut sides = Vec::with_capacity(points.len());
    for (i, &a) in points.iter().enumerate() {
        sides.push(make::edge_between(m, a, points[(i + 1) % points.len()])?);
    }
    let outline = make::wire(m, &sides)?;
    let section = make::planar_face(m, outline)?;
    Ok(sweep::extrude(m, section, v)?.shape)
}
