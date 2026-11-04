//! The L-shaped plate, the polygon (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15) extruded
//! 8 up, shrunk by 1: every point at least 1 from its outside. Each face moves 1 inward and the
//! convex edges stay sharp; the concave edge along (5, 5) becomes a concave quarter cylinder of
//! radius 1 about it.
//!
//! geop has no offset of a solid that rounds where moved faces part: its face offset moves the
//! faces and meets them sharp everywhere, the two beside the concave edge in the line x = 4,
//! y = 4. So the plate is shrunk that way, and that edge then rounded to radius 1: the round
//! tangent to x = 4 and y = 4 on the material's side is the cylinder of radius 1 about the line
//! x = 5, y = 5, the concave edge itself.
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
    offset_faces(b, "shrink", &faces, -1.0)?;
    let corner = b.edge([4.0, 4.0, 4.0])?;
    b.step("round", FilletArgs::constant(vec![corner], 1.0))
}
