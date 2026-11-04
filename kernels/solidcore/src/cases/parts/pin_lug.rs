//! A flat lug, the plate from (0, −5, 0) to (80, 5, 40), with a pin hole of radius 8 through its
//! thickness along y about the line x = 60, z = 20: a cylinder running past both faces, cut.
use crate::bench::{cut, rotated, translated};
use crate::Refusal;
use solidcore_ops::{make_box, make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let plate = make_box([0.0, -5.0, 0.0], [80.0, 10.0, 40.0])?;
    // a cylinder about z from −7 to 7, turned onto y, moved onto the hole's axis
    let pin = make_cylinder([0.0, 0.0, -7.0], 8.0, 14.0)?;
    let pin = translated(rotated(pin, [0.0; 3], [1.0, 0.0, 0.0], -90.0)?, [60.0, 0.0, 20.0])?;
    cut(plate, pin)
}
