//! The block on the square −12 ≤ x, y ≤ 12 with the weighted barrel-vaulted top of
//! `nurbs/groin-vault` (an arch of radius 15 about the line y = 0, z = 1, its crown z = 16 along
//! y = 0), with everything above z = 16 cut away by a box whose floor is the plane z = 16 and which
//! reaches past the block on every other side. The box's floor touches the vault along its whole
//! crown, so the cut removes nothing.
use crate::bench::p;
use crate::shared::nurbs::barrel_vault;
use kentron::boolean;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let block = barrel_vault(m)?;
    let above = Cuboid::from_corners(p(-24.0, -24.0, 16.0), p(24.0, 24.0, 40.0)).build(m)?;
    boolean::difference(m, block, above)
}
