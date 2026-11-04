//! The floor and four walls of the cube from the origin to (10, 10, 10), with no top, an open
//! tray, thickened by 1 out of the cube, the moved faces extended until they meet: the box from
//! (-1, -1, -1) to (11, 11, 10) less the cube.
use crate::shared::thicken::open_box;
use crate::Refusal;
use solidcore_ops::{thicken, Body};

pub fn build() -> Result<Body, Refusal> {
    let tray = open_box()?;
    // its faces' normals face out of the cube; ops's thicken extends the parted faces
    thicken(&tray, 1.0, false)?;
    Ok(tray)
}
