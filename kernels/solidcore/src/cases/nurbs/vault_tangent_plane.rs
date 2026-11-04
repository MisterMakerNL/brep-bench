//! A block on the square −12 ≤ x, y ≤ 12 with a barrel-vaulted top (the rational B-spline of
//! degree 1 along x and 2 along y through (x, −12, 10), (x, 0, 26) and (x, 12, 10), weighted 1,
//! 3/5 and 1), from which everything above z = 16 is cut away by a box whose floor is that plane,
//! tangent to the vault along its crown: the cut removes nothing.
use crate::bench::cut;
use crate::shared::nurbs::vault;
use crate::Refusal;
use solidcore_ops::{make_box, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = vault()?;
    let above = make_box([-20.0, -20.0, 16.0], [40.0, 40.0, 20.0])?;
    cut(block, above)
}
