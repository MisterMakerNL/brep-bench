//! The box from the origin to (10, 8, 6), shrunk by 1: each face moves 1 inward and the edges
//! stay sharp, leaving the box from (1, 1, 1) to (9, 7, 5). The box is extruded from its
//! rectangle and its six faces offset 1 into it by geop's face offset, which rebuilds the solid
//! where the moved faces meet; each face is picked by a point on it.
use crate::shared::hollow::{block, offset_faces};
use crate::{Bench, Refusal};

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
    offset_faces(b, "shrink", &faces, -1.0)
}
