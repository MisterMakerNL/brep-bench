//! The tray, five faces of the cube from the origin to (10, 10, 10) open at the top, thickened by
//! 1 into the cube, the moved faces trimmed where they cross: the cube less the box from
//! (1, 1, 1) to (9, 9, 10).
use crate::shared::thicken::{thickened, tray};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    // the tray's faces face out of the cube: against them
    thickened(tray()?, -1.0, false)
}
