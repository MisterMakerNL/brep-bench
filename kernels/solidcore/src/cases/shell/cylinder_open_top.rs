//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, shelled inward to a wall 1 thick,
//! open at its top face: a cup, the cylinder less the cylinder of radius 4 from z = 1 to z = 10.
use crate::bench::{face_through, order};
use crate::Refusal;
use solidcore_ops::{make_cylinder, shell, Body};

pub fn build() -> Result<Body, Refusal> {
    let drum = make_cylinder([0.0, 0.0, 0.0], 5.0, 10.0)?;
    let top = face_through(&drum, [0.0, 0.0, 10.0])?;
    shell(&drum, &order(&drum)?, &[top], -1.0)?;
    Ok(drum)
}
