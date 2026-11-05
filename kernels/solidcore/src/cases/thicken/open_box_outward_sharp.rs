//! The tray, five faces of the cube from the origin to (10, 10, 10) open at the top, thickened by
//! 1 out of the cube, the moved faces extended to meet: the box from (−1, −1, −1) to
//! (11, 11, 10) less the cube.
use crate::shared::thicken::{thickened, tray};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    // the tray's faces face out of the cube: along them
    thickened(tray()?, 1.0, false)
}
