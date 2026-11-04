//! A plate, the box from the origin to (40, 30, 8), drilled right through along z by a hole of
//! radius 5 on the vertical line through (20, 15); then the hole's wall offset by 2 into the
//! plate, away from its axis, to the cylinder of radius 7, the top and bottom trimmed back to it.
use crate::shared::edit::{drilled_box, move_faces};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let plate = drilled_box([40.0, 30.0, 8.0], 20.0, 15.0, 5.0)?;
    // the wall's outward normal points into the hole, toward the axis: into the plate is inward
    move_faces(&plate, &[[20.0, 20.0, 4.0]], -2.0)?;
    Ok(plate)
}
