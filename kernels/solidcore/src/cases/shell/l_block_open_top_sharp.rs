//! The L-shaped plate (the polygon through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15),
//! extruded 8 along +z) shelled inward to a wall 1 thick, open at its top, z = 8, every pair of
//! moved faces extended until they meet, the concave edge's included: the cavity is the polygon
//! through (1, 1), (19, 1), (19, 4), (4, 4), (4, 14) and (1, 14) from z = 1 up. Ops's shell
//! extends the moved faces until they meet.
use crate::bench::{face_through, order, polygon};
use crate::Refusal;
use solidcore_ops::sweep::{extrude, DraftCorner};
use solidcore_ops::{shell, Body};

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
    shell(&plate, &order(&plate)?, &[top], -1.0)?;
    Ok(plate)
}
