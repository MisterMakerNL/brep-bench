//! What the chamfer implementations share.
use crate::bench::p;
use crate::Refusal;
use kentron::make;
use kentron::math::{Point, Vector};
use kentron::sew::{self, SewOptions};
use kentron::sweep;
use kentron::topology::{Model, NodeKind, Shape};

/// A planar face bounded by the closed polygon through `points`.
fn polygon(m: &mut Model, points: &[Point]) -> kentron::Result<Shape> {
    let mut sides = Vec::with_capacity(points.len());
    for (i, &a) in points.iter().enumerate() {
        sides.push(make::edge_between(m, a, points[(i + 1) % points.len()])?);
    }
    let outline = make::wire(m, &sides)?;
    make::planar_face(m, outline)
}

/// The prism of the closed polygon through `points`, extruded along `v`.
pub fn prism(m: &mut Model, points: &[Point], v: Vector) -> kentron::Result<Shape> {
    let section = polygon(m, points)?;
    Ok(sweep::extrude(m, section, v)?.shape)
}

/// The octahedron |x| + |y| + |z| ≤ `size`: its eight triangles sewn into a shell, the shell
/// made a solid.
pub fn octahedron(m: &mut Model, size: f64) -> Result<Shape, Refusal> {
    let mut faces = Vec::new();
    for sx in [1.0, -1.0] {
        for sy in [1.0, -1.0] {
            for sz in [1.0, -1.0] {
                let corners = [p(sx * size, 0.0, 0.0), p(0.0, sy * size, 0.0), p(0.0, 0.0, sz * size)];
                faces.push(polygon(m, &corners)?);
            }
        }
    }
    let shell = sew::sew(m, &faces, &SewOptions::default())?;
    if shell.kind() != NodeKind::Shell {
        return Err(format!("the octahedron's faces sewed into a {:?}, not a shell", shell.kind()).into());
    }
    let solid = m.new_empty(NodeKind::Solid);
    m.add(solid, shell)?;
    Ok(solid)
}
