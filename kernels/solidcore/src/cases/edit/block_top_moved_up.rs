//! A block, the box from the origin to (40, 30, 10), drilled right through along z by a hole of
//! radius 5 on the vertical line through (20, 15); then its top face moved up by 6 to z = 16, the
//! sides and the hole's wall running on up to meet it.
use crate::shared::edit::{drilled_box, move_faces};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let block = drilled_box([40.0, 30.0, 10.0], 20.0, 15.0, 5.0)?;
    // the top, by a point on it clear of the hole; up is its outward normal
    move_faces(&block, &[[5.0, 5.0, 10.0]], 6.0)?;
    Ok(block)
}
