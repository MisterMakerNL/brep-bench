//! The box from the origin to (20, 14, 10) with its four vertical edges rounded to radius 2,
//! shelled inward to a wall 1 thick, open at its top, z = 10: a tray whose inside corners are
//! rounded to radius 1, concentric with its outside ones.
use crate::bench::{face_through, order};
use crate::shared::shell::rounded_plate;
use crate::Refusal;
use solidcore_ops::{shell, Body};

pub fn build() -> Result<Body, Refusal> {
    let plate = rounded_plate()?;
    let top = face_through(&plate, [10.0, 7.0, 10.0])?;
    shell(&plate, &order(&plate)?, &[top], -1.0)?;
    Ok(plate)
}
