//! The box from the origin to (40, 30, 10), drilled right through along z by a hole of radius 5
//! about the vertical line through (20, 15). Its top face, the plane z = 10, is moved up by 6, to
//! z = 16; the four sides and the hole's wall run on up to meet it.
use crate::bench::{face_through, p};
use crate::shared::edit::{drilled_block, moved};
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = drilled_block(m, [40.0, 30.0, 10.0], 20.0, 15.0, 5.0)?;
    let top = face_through(m, block, p(5.0, 5.0, 10.0))?;
    Ok(moved(m, block, &[(top, 6.0)])?) // its outward normal is +z
}
