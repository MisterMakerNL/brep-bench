//! A block on the square 0 ≤ x, y ≤ 20, its floor flat at z = 0, its sides upright, and its top
//! the bicubic Bézier patch whose control points stand over an even 4 × 4 grid of the square at
//! the heights 4 6 7 5 / 6 14 12 7 / 5 13 16 8 / 3 6 8 6 (row i at x = 20i/3, column j at
//! y = 20j/3). The top face is swept straight down past the floor and cut off at the floor.
use crate::shared::nurbs::bezier_dome;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    bezier_dome(m)
}
