//! The octahedron |x| + |y| + |z| ≤ 10, all twelve edges chamfered with the same distance √6 on
//! both faces (measured across each face, square to the edge), the corners mitred: each edge its
//! own chamfer, one after another, ending on the faces at its ends (the chamfers before it among
//! them), so the four at each corner meet in a point. The octahedron cut by the twelve planes
//! |x| + |y| ≤ 8 and the like.
use crate::bench::{edge_through, p};
use crate::shared::chamfer::octahedron;
use crate::Refusal;
use kentron::blend;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let mut solid = octahedron(m, 10.0)?;
    // Each edge picked by its middle, such as (5, 5, 0) for the edge from (10, 0, 0) to
    // (0, 10, 0): what is left of an edge keeps its middle.
    let mut middles = Vec::new();
    for a in [5.0, -5.0] {
        for b in [5.0, -5.0] {
            middles.extend([p(a, b, 0.0), p(0.0, a, b), p(b, 0.0, a)]);
        }
    }
    for middle in middles {
        let edge = edge_through(m, solid, middle)?;
        solid = blend::chamfer(m, solid, &[edge], 6f64.sqrt())?;
    }
    Ok(solid)
}
