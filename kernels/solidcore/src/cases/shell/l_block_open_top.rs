//! The L-shaped plate (the polygon through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15),
//! extruded 8 along +z) shelled inward to a wall 1 thick, open at its top, z = 8, the wall every
//! point within 1 of the other faces: sharp along the convex edges, and at the concave edge a
//! quarter cylinder of radius 1 about it. The kernel's shell with the moved faces joined by
//! rounds where they part.
use crate::bench::{face_through, polygon};
use crate::shared::shell::shell_rounded;
use crate::Refusal;
use solidcore_ops::sweep::{extrude, DraftCorner};
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let outline = polygon(&[
        [0.0, 0.0, 0.0],
        [20.0, 0.0, 0.0],
        [20.0, 5.0, 0.0],
        [5.0, 5.0, 0.0],
        [5.0, 15.0, 0.0],
        [0.0, 15.0, 0.0],
    ]);
    let plate = extrude(&outline, [0.0, 0.0, 1.0], 8.0, 0.0, DraftCorner::default(), true)?;
    let top = face_through(&plate, [2.0, 2.0, 8.0])?;
    shell_rounded(&plate, &[top], -1.0)
}
