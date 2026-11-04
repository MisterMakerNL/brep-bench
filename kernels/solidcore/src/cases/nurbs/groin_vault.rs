//! The common part of two blocks with barrel-vaulted tops on the square −12 ≤ x, y ≤ 12, the
//! second the first turned a quarter turn about the z axis: a groin vault. Each block's top is the
//! rational B-spline of degree 1 along x and 2 along y through (x, −12, 10), (x, 0, 26) and
//! (x, 12, 10), weighted 1, 3/5 and 1: the block's arched section extruded along x.
use crate::bench::{common, rotated};
use crate::shared::nurbs::vault;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let first = vault()?;
    let second = rotated(vault()?, [0.0; 3], [0.0, 0.0, 1.0], 90.0)?;
    common(first, second)
}
