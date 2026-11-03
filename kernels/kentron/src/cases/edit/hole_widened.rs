//! The box from the origin to (40, 30, 8), drilled right through along z by a hole of radius 5
//! about the vertical line through (20, 15). The hole's wall is offset 2 into the plate, away from
//! its axis, to the cylinder of radius 7; the top and the bottom are trimmed back to meet it.
use crate::bench::{face_through, p};
use crate::shared::edit::{drilled_block, moved};
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let plate = drilled_block(m, [40.0, 30.0, 8.0], 20.0, 15.0, 5.0)?;
    let wall = face_through(m, plate, p(25.0, 15.0, 4.0))?;
    // The wall's outward normal points out of the plate, into the hole, toward its axis: into
    // the plate, away from the axis, is the other way.
    Ok(moved(m, plate, &[(wall, -2.0)])?)
}
