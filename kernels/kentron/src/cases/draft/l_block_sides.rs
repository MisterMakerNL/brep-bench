//! An L-block, the outline through (0, 0), (30, 0), (30, 10), (10, 10), (10, 25) and (0, 25)
//! extruded from z = 0 to z = 12, all six sides drafted 6° with the pull direction +z about the
//! neutral plane z = 0, each leaning into the material as it rises, the two at the concave edge
//! too: its section at height z is the L moved z·tan 6° into its material.
use crate::bench::{face_through, p, v};
use crate::shared::draft::drafted;
use crate::Refusal;
use kentron::make;
use kentron::math::Direction;
use kentron::sweep;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let outline = [(0.0, 0.0), (30.0, 0.0), (30.0, 10.0), (10.0, 10.0), (10.0, 25.0), (0.0, 25.0)];
    let mut sides = Vec::new();
    for (i, &(x, y)) in outline.iter().enumerate() {
        let (nx, ny) = outline[(i + 1) % outline.len()];
        sides.push(make::edge_between(m, p(x, y, 0.0), p(nx, ny, 0.0))?);
    }
    let wire = make::wire(m, &sides)?;
    let floor = make::planar_face(m, wire)?;
    let block = sweep::extrude(m, floor, v(0.0, 0.0, 12.0))?.shape;
    // Each side picked at the middle of its bottom edge, halfway up.
    let mut faces = Vec::new();
    for (i, &(x, y)) in outline.iter().enumerate() {
        let (nx, ny) = outline[(i + 1) % outline.len()];
        faces.push(face_through(m, block, p((x + nx) / 2.0, (y + ny) / 2.0, 6.0))?);
    }
    drafted(m, block, &faces, 6.0, p(0.0, 0.0, 0.0), Direction::Z)
}
