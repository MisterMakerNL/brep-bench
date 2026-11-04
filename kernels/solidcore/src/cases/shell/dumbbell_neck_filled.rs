//! A dumbbell plate, the outline through (0, 0), (10, 0), (10, 4), (14, 4), (14, 0), (24, 0),
//! (24, 10), (14, 10), (14, 6), (10, 6), (10, 10) and (0, 10) extruded 6 along +z, shelled inward
//! to a wall 1.5 thick, open at its top: the wall every point within 1.5 of the other faces. The
//! neck, 2 wide, fills, and each pocket bulges towards it in arcs about the neck's corners: the
//! kernel's shell with the moved faces joined by rounds where they part.
use crate::bench::{face_through, polygon};
use crate::shared::shell::shell_rounded;
use crate::Refusal;
use solidcore_ops::sweep::{extrude, DraftCorner};
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let outline = polygon(&[
        [0.0, 0.0, 0.0],
        [10.0, 0.0, 0.0],
        [10.0, 4.0, 0.0],
        [14.0, 4.0, 0.0],
        [14.0, 0.0, 0.0],
        [24.0, 0.0, 0.0],
        [24.0, 10.0, 0.0],
        [14.0, 10.0, 0.0],
        [14.0, 6.0, 0.0],
        [10.0, 6.0, 0.0],
        [10.0, 10.0, 0.0],
        [0.0, 10.0, 0.0],
    ]);
    let plate = extrude(&outline, [0.0, 0.0, 1.0], 6.0, 0.0, DraftCorner::default(), true)?;
    let top = face_through(&plate, [5.0, 5.0, 6.0])?;
    shell_rounded(&plate, &[top], -1.5)
}
