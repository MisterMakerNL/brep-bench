//! A picture frame: the rebated moulding (s outward, z) = (0, 2), (3, 2), (3, 0), (8, 0), (8, 6),
//! (0, 6) swept round the closed rectangle with corners (±30, ±20, 0), mitred at each corner in
//! the vertical plane along its diagonal.
use crate::sketch::{mitred, polygon};
use cad_kernel::topo::Brep;

const SECTION: [[f64; 2]; 6] = [[0., 2.], [3., 2.], [3., 0.], [8., 0.], [8., 6.], [0., 6.]];
const HALF_WIDTH: f64 = 30.;
const HALF_HEIGHT: f64 = 20.;

pub fn build() -> Result<Brep, String> {
    let (w, h) = (HALF_WIDTH, HALF_HEIGHT);
    // round the rectangle anticlockwise from its corner (−30, −20), first along the side y = −20
    let path = [[-w, -h, 0.], [w, -h, 0.], [w, h, 0.], [-w, h, 0.]];
    // the section where it starts, square to that side: (s, z) at (−30, −20 − s, z)
    let points: Vec<_> = SECTION.iter().map(|&[s, z]| [-w, -h - s, z]).collect();
    let reach = SECTION.iter().map(|&[s, z]| s.hypot(z)).fold(0., f64::max);
    mitred(&polygon(&points), &path, true, reach)
}
