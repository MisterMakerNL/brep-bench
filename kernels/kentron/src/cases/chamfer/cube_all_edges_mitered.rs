//! A cube of side 10 between the origin and (10, 10, 10), all twelve edges chamfered with the
//! same distance 2 on both faces, the corners mitred: each edge its own chamfer, one after
//! another, ending on the faces at its ends (the chamfers before it among them), so the three
//! at each corner meet in a point. The cube cut by the twelve planes x + y ≤ 18 and the like.
use crate::bench::{edge_through, p};
use crate::Refusal;
use kentron::blend;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let mut solid = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 10.0, 10.0)).build(m)?;
    // Each edge picked by its middle, one coordinate 5 and the other two 0 or 10: what is left of
    // an edge keeps its middle, the chamfers before it cutting as much off either end.
    let mut middles = Vec::new();
    for u in [0.0, 10.0] {
        for w in [0.0, 10.0] {
            middles.extend([p(5.0, u, w), p(u, 5.0, w), p(u, w, 5.0)]);
        }
    }
    for middle in middles {
        let edge = edge_through(m, solid, middle)?;
        solid = blend::chamfer(m, solid, &[edge], 2.0)?;
    }
    Ok(solid)
}
