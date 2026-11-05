//! A square bar 10 × 10 swept along two straight runs, (0, 0, 0) to (40, 0, 0) and on, turned
//! 120° to the left, to (20, 20√3, 0), mitred at the corner in the vertical plane bisecting it;
//! the section square to the path, two sides horizontal, the ends flat.
use crate::sketch::{mitred, polygon};
use cad_kernel::topo::Brep;

const HALF: f64 = 5.;
const RUN: f64 = 40.;

pub fn build() -> Result<Brep, String> {
    let turn = 120f64.to_radians(); // to the left, from +x towards +y
    let path = [[0., 0., 0.], [RUN, 0., 0.], [RUN + RUN * turn.cos(), RUN * turn.sin(), 0.]];
    // the square at the path's start, in the plane x = 0, its sides at y, z = ±5 (turned at the
    // corner about the vertical, it stays upright)
    let square = polygon(&[[0., -HALF, -HALF], [0., HALF, -HALF], [0., HALF, HALF], [0., -HALF, HALF]]);
    mitred(&square, &path, false, HALF * std::f64::consts::SQRT_2)
}
