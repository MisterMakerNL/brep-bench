//! The L-shaped plate, the polygon (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15) extruded
//! 8 up, grown by 1 with its edges kept sharp: every face moved 1 outward, the moved faces
//! extended until they meet. The plate is extruded from its outline and its eight faces offset
//! 1 out of it by geop's face offset, which rebuilds the solid where the moved faces meet; each
//! face is picked by a point on it.
use crate::shared::hollow::{offset_faces, prism};
use crate::{Bench, Refusal};

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
    offset_faces(b, "grow", &faces, 1.0)
}
