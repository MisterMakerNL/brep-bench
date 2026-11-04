//! The sphere of radius 5 about the origin as a surface alone, one closed face with no solid
//! inside it, thickened by 1 outward: the hollow ball between radii 5 and 6.
use crate::bench::{arc, order};
use crate::Refusal;
use solidcore_ops::sweep::revolve;
use solidcore_ops::{shell, Body};
use std::f64::consts::{PI, TAU};

pub fn build() -> Result<Body, Refusal> {
    // the half circle in the xz plane from the south pole through (5, 0, 0) to the north pole,
    // turned a full turn about the z axis uncapped: the sphere's face alone
    let meridian = arc([0.0; 3], [0.0, -1.0, 0.0], [1.0, 0.0, 0.0], 5.0, -PI / 2.0, PI / 2.0);
    let skin = revolve(&[meridian], [0.0; 3], [0.0, 0.0, 1.0], TAU, false)?;
    // solidcore has no closed sheet: a face closing on itself bounds a solid, the ball (and
    // ops's thicken refuses a solid). Thickening a closed sheet outward is shelling it with no
    // face opened, its wall outward: the face kept, its offset outside it.
    shell(&skin, &order(&skin)?, &[], 1.0)?;
    Ok(skin)
}
