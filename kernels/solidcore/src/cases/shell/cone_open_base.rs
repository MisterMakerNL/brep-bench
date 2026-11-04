//! A solid cone on the z axis, its base the disc of radius 6 at z = 0 and its apex at (0, 0, 8),
//! shelled inward to a wall 1 thick, open at its base: the cone less the cone of the same angle
//! whose apex is 5/3 lower.
use crate::bench::{face_through, order};
use crate::Refusal;
use solidcore_ops::{make_cone, shell, Body};

pub fn build() -> Result<Body, Refusal> {
    let cone = make_cone([0.0, 0.0, 0.0], 6.0, 0.0, 8.0)?;
    let base = face_through(&cone, [0.0, 0.0, 0.0])?;
    shell(&cone, &order(&cone)?, &[base], -1.0)?;
    Ok(cone)
}
