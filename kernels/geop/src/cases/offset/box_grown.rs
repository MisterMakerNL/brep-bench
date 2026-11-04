//! The box from the origin to (10, 8, 6), grown by 1: every point within 1 of it. Each face
//! moves 1 outward, each edge becomes a quarter cylinder of radius 1 about it and each corner an
//! eighth of a ball of radius 1 about it.
//!
//! geop has no offset of a solid that rounds where moved faces part: its face offset moves the
//! faces and meets them sharp. So the box's six faces are offset 1 out, to the box from
//! (−1, −1, −1) to (11, 9, 7), and its twelve edges then rounded to radius 1: each round is
//! tangent to two moved faces, so it is the quarter cylinder of radius 1 about the box's own
//! edge, and where three meet, the eighth of a ball about the box's corner.
use crate::shared::hollow::{block, offset_faces};
use crate::{Bench, Refusal};
use geop_ops_fillet::FilletArgs;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    block(b, "box", [0.0, 0.0], [10.0, 8.0], 6.0)?;
    let faces = [
        [5.0, 4.0, 0.0],
        [5.0, 4.0, 6.0],
        [5.0, 0.0, 3.0],
        [5.0, 8.0, 3.0],
        [0.0, 4.0, 3.0],
        [10.0, 4.0, 3.0],
    ];
    offset_faces(b, "grow", &faces, 1.0)?;
    let mut edges = Vec::new();
    for z in [-1.0, 7.0] {
        edges.extend([[5.0, -1.0, z], [11.0, 4.0, z], [5.0, 9.0, z], [-1.0, 4.0, z]]);
    }
    edges.extend([[-1.0, -1.0, 3.0], [11.0, -1.0, 3.0], [11.0, 9.0, 3.0], [-1.0, 9.0, 3.0]]);
    let edges = b.edges(&edges)?;
    b.step("rounds", FilletArgs::constant(edges, 1.0))
}
