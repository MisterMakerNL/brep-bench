//! The box from the origin to (20, 14, 10) with its four vertical edges rounded to radius 2,
//! shelled inward to a wall 2 thick, open at its top, z = 10: each round, moved 2 in, shrinks to
//! its axis, where the inside walls meet in a sharp edge; the cavity is the box from (2, 2, 2) to
//! (18, 12, 10).
use crate::bench::{face_through, order};
use crate::shared::shell::rounded_plate;
use crate::Refusal;
use solidcore_ops::{shell, Body};

pub fn build() -> Result<Body, Refusal> {
    let plate = rounded_plate()?;
    let top = face_through(&plate, [10.0, 7.0, 10.0])?;
    shell(&plate, &order(&plate)?, &[top], -2.0)?;
    Ok(plate)
}
