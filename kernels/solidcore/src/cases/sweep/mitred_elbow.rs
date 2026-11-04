//! A square bar, its section the square 10 × 10 square to the path and centred on it, two sides
//! horizontal, swept along the path from (0, 0, 0) along +x to (40, 0, 0), then turning 120° to
//! the left and on 40 to (20, 20√3, 0), the runs mitred at the corner in the plane that bisects
//! it, x + √3y = 40; its ends flat and square to the path.
use crate::bench::{line, polygon};
use crate::shared::sweep::mitred;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    // the section at the path's start, square to the first run
    let section =
        polygon(&[[0.0, -5.0, -5.0], [0.0, 5.0, -5.0], [0.0, 5.0, 5.0], [0.0, -5.0, 5.0]]);
    let corner = [40.0, 0.0, 0.0];
    let path = [line([0.0; 3], corner), line(corner, [20.0, 20.0 * 3f64.sqrt(), 0.0])];
    mitred(&section, &path)
}
