//! The sphere of radius 5 about the origin as a surface alone, a sheet of one closed face with
//! no solid inside it, thickened by 1 outward: the hollow ball between radii 5 and 6.
//!
//! The sheet is the half circle from (0, 0, −5) through (5, 0, 0) to (0, 0, 5) turned a whole
//! turn about the z axis without caps. A face that closes on itself bounds a region in
//! solidcore, so the kernel takes it for a solid, and ops's `thicken`, which thickens sheets,
//! refuses it. A closed sheet thickened with nothing open is the kernel's shell of it, with no
//! face opened: the wall it grows outward, and the face itself left bounding the void inside.
use crate::bench::arc;
use crate::bench::order;
use crate::Refusal;
use solidcore_ops::sweep::revolve;
use solidcore_ops::{shell, Body};
use std::f64::consts::{FRAC_PI_2, TAU};

pub fn build() -> Result<Body, Refusal> {
    // about −y, angle 0 along +x: from straight below the center round through +x to above it
    let meridian = arc([0.0; 3], [0.0, -1.0, 0.0], [1.0, 0.0, 0.0], 5.0, -FRAC_PI_2, FRAC_PI_2);
    let skin = revolve(&[meridian], [0.0; 3], [0.0, 0.0, 1.0], TAU, false)?;
    shell(&skin, &order(&skin)?, &[], 1.0)?;
    Ok(skin)
}
