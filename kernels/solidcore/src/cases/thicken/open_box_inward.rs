//! The floor and four walls of the cube from the origin to (10, 10, 10), with no top, an open
//! tray, thickened by 1 into the cube: the cube less the box from (1, 1, 1) to (9, 9, 10).
use crate::shared::thicken::open_box;
use crate::Refusal;
use solidcore_ops::{thicken, Body};

pub fn build() -> Result<Body, Refusal> {
    let tray = open_box()?;
    // its faces' normals face out of the cube: into it is against them
    thicken(&tray, -1.0, false)?;
    Ok(tray)
}
