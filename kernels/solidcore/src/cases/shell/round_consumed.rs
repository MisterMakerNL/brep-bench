//! The box from the origin to (20, 14, 10) with its four vertical edges rounded to radius 2,
//! shelled inward to a wall 3 thick, open at its top, z = 10: each round, moved 3 in, would turn
//! inside out, so the inside walls beside it run on to a sharp edge and the cavity is the box from
//! (3, 3, 3) to (17, 11, 10).
use crate::bench::{face_through, order};
use crate::shared::shell::rounded_plate;
use crate::Refusal;
use solidcore_ops::{shell, Body};

pub fn build() -> Result<Body, Refusal> {
    let plate = rounded_plate()?;
    let top = face_through(&plate, [10.0, 7.0, 10.0])?;
    shell(&plate, &order(&plate)?, &[top], -3.0)?;
    Ok(plate)
}
