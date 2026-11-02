//! A block on the square 0 ≤ x, y ≤ 20, its floor flat at z = 0, its sides upright, and its top
//! the non-rational Bézier patch of degree 15 each way whose control point Pᵢⱼ is
//! (20i/15, 20j/15, 4 + ((3i + 5j) mod 7)). The top face is swept straight down past the floor,
//! and the column cut off at the floor by a block wider than the square.
use crate::bench::{p, v};
use kentron::boolean;
use kentron::geometry::surface::{BezierSurface, Surface};
use kentron::make;
use kentron::primitives::Cuboid;
use kentron::sweep;
use kentron::topology::{Model, Shape};

const SIDE: f64 = 20.0;
const DEGREE: usize = 15;

fn height(i: usize, j: usize) -> f64 {
    4.0 + ((3 * i + 5 * j) % 7) as f64
}

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let step = SIDE / DEGREE as f64;
    let poles = (0..=DEGREE)
        .map(|i| (0..=DEGREE).map(|j| p(step * i as f64, step * j as f64, height(i, j))).collect())
        .collect();
    let top = make::face_on_surface(m, Surface::Bezier(BezierSurface::new(poles)))?;
    let column = sweep::extrude(m, top, v(0.0, 0.0, -2.0 * SIDE))?.shape;
    let floor_up = Cuboid::from_corners(p(-1.0, -1.0, 0.0), p(SIDE + 1.0, SIDE + 1.0, 2.0 * SIDE)).build(m)?;
    boolean::intersection(m, column, floor_up)
}
