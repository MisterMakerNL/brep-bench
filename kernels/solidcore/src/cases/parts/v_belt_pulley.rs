//! A V-belt pulley about the z axis: a hub (radius 12, z from −14 to 14) and a rim (radius 28, z
//! from −9 to 9) united, bored through by radius 6, and its belt groove taken away: the triangle
//! (22, 0, 0), (29, 0, −5), (29, 0, 5) of the xz plane, reaching past the rim, turned a whole turn
//! about the z axis.
use crate::bench::{cut, fuse, polygon};
use crate::Refusal;
use solidcore_ops::sweep::revolve;
use solidcore_ops::{make_cylinder, Body};
use std::f64::consts::TAU;

pub fn build() -> Result<Body, Refusal> {
    let hub = make_cylinder([0.0, 0.0, -14.0], 12.0, 28.0)?;
    let rim = make_cylinder([0.0, 0.0, -9.0], 28.0, 18.0)?;
    let bore = make_cylinder([0.0, 0.0, -15.0], 6.0, 30.0)?;
    let groove = revolve(
        &polygon(&[[22.0, 0.0, 0.0], [29.0, 0.0, -5.0], [29.0, 0.0, 5.0]]),
        [0.0; 3],
        [0.0, 0.0, 1.0],
        TAU,
        true,
    )?;
    cut(cut(fuse(hub, rim)?, bore)?, groove)
}
