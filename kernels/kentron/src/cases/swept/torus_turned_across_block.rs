//! A curved block, the sector of the ring about the z axis between radii 8 and 12, z from −1 to
//! 1 and polar angle from 0° to 60°, grooved by a leaning cutter turned past it: the torus centred
//! at (5, 0, 0) with radii 2.5 and 2, its axis (0, sin 25°, cos 25°), turned rigidly about the z
//! axis from −140° to +140°. Everything the cutter passes through, cut.
//!
//! Refused. kentron turns a shape about an axis (`sweep::revolve`), but no solid. It turns the
//! cutter's face whole, and a face turned whole is what it passes through only if the turn never
//! runs along it inside it. On this torus the turn runs along it on its characteristic curve,
//! which is none of its circles, since its axis is skew to the turn's. Splitting the face along
//! that curve first would be computing the envelope that walls the groove outside the kernel, and
//! kentron has no operation to find it.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "kentron has no sweep of a solid about an axis: sweep::revolve takes no solid and turns the \
         torus's face whole, and the groove's wall is the envelope of the turned torus, along a \
         characteristic curve that is none of its circles, which kentron has no operation to find",
    ))
}
