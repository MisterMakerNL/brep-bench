//! An open tray lying on its back, opening toward −y: the block (−36, 0, −24) to (36, 16, 24), less
//! the pocket (−33, 0, −21) to (33, 13, 21); then four standoffs joined to it on the floor's
//! inside face (y = 13), each a cylinder of radius 5 along y from y = 5 to y = 13 about
//! (x, z) = (±26, ±14), and each bored blind by radius 2 from past its open end down to the
//! floor, which is left whole.
use crate::bench::{cut, cut_all, fuse_all, rotated, translated};
use crate::Refusal;
use solidcore_ops::{make_box, make_cylinder, Body};

/// The cylinder of `radius` along +y from y = `y0` to y = `y1` about (x, z).
fn along_y(x: f64, z: f64, radius: f64, y0: f64, y1: f64) -> Result<Body, Refusal> {
    // about z from 0 to the length, turned so that +z goes to +y, moved into place
    let c = make_cylinder([0.0; 3], radius, y1 - y0)?;
    translated(rotated(c, [0.0; 3], [1.0, 0.0, 0.0], -90.0)?, [x, y0, z])
}

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([-36.0, 0.0, -24.0], [72.0, 16.0, 48.0])?;
    let pocket = make_box([-33.0, 0.0, -21.0], [66.0, 13.0, 42.0])?;
    let tray = cut(block, pocket)?;
    let feet = [(26.0, 14.0), (-26.0, 14.0), (-26.0, -14.0), (26.0, -14.0)];
    let mut parts = vec![tray];
    for &(x, z) in &feet {
        parts.push(along_y(x, z, 5.0, 5.0, 13.0)?);
    }
    let joined = fuse_all(parts)?;
    let bores = feet.iter().map(|&(x, z)| along_y(x, z, 2.0, 4.0, 13.0)).collect::<Result<Vec<_>, _>>()?;
    cut_all(joined, bores)
}
