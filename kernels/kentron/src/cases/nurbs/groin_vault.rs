//! A groin vault: the common part of two blocks on the square −12 ≤ x, y ≤ 12 with barrel-vaulted
//! tops, the second the first turned a quarter turn about the z axis. Each top is the weighted
//! rational B-spline surface, degree 1 along x and 2 along y, whose rows are the arch of radius 15
//! about the line y = 0, z = 1 (control points (x, −12, 10), (x, 0, 26), (x, 12, 10), weights 1,
//! 3/5, 1). The vaults meet along the diagonals, tangent to each other at the crown (0, 0, 16).
use crate::shared::nurbs::barrel_vault;
use kentron::boolean;
use kentron::math::{Axis, Transform};
use kentron::modify;
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_2;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let first = barrel_vault(m)?;
    let quarter = Transform::rotation(Axis::Z, FRAC_PI_2);
    let second = modify::transform(m, first, &quarter, Default::default())?;
    boolean::intersection(m, first, second)
}
