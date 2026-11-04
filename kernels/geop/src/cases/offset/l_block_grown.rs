//! The L-shaped plate, the polygon (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15) extruded
//! 8 up, grown by 1: every point within 1 of it. Each face moves 1 outward, each convex edge
//! becomes a quarter cylinder of radius 1 about it and each convex corner an eighth of a ball;
//! the concave edge stays sharp, and the rounds of the top and bottom edges beside it meet in a
//! sharp crease.
//!
//! geop has no offset of a solid that rounds where moved faces part: its face offset moves the
//! faces and meets them sharp. So the plate's faces are offset 1 out, and then its convex edges
//! (all but the one along (6, 6)) rounded to radius 1: each round is tangent to two moved
//! faces, so it lies about the plate's own edge.
use crate::shared::hollow::{offset_faces, prism};
use crate::{Bench, Refusal};
use geop_ops_fillet::FilletArgs;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let outline = [[0.0, 0.0], [20.0, 0.0], [20.0, 5.0], [5.0, 5.0], [5.0, 15.0], [0.0, 15.0]];
    prism(b, "plate", &outline, 8.0)?;
    let faces = [
        [2.0, 2.0, 0.0],
        [2.0, 2.0, 8.0],
        [10.0, 0.0, 4.0],
        [20.0, 2.0, 4.0],
        [10.0, 5.0, 4.0],
        [5.0, 10.0, 4.0],
        [2.0, 15.0, 4.0],
        [0.0, 7.0, 4.0],
    ];
    offset_faces(b, "grow", &faces, 1.0)?;
    // The grown outline: (−1, −1), (21, −1), (21, 6), (6, 6), (6, 16), (−1, 16).
    let mut edges = Vec::new();
    for z in [-1.0, 9.0] {
        edges.extend([
            [10.0, -1.0, z],
            [21.0, 2.0, z],
            [12.0, 6.0, z],
            [6.0, 10.0, z],
            [2.0, 16.0, z],
            [-1.0, 7.0, z],
        ]);
    }
    edges.extend([
        [-1.0, -1.0, 4.0],
        [21.0, -1.0, 4.0],
        [21.0, 6.0, 4.0],
        [6.0, 16.0, 4.0],
        [-1.0, 16.0, 4.0],
    ]);
    let edges = b.edges(&edges)?;
    b.step("rounds", FilletArgs::constant(edges, 1.0))
}
