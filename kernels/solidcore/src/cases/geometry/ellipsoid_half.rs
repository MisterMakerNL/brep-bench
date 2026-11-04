//! The ellipsoid x²/36 + y²/16 + z²/9 ≤ 1 with everything below z = 0 cut away by a block reaching
//! past it everywhere but at z = 0: a dome on the flat ellipse x²/36 + y²/16 ≤ 1. solidcore has no
//! ellipsoid, so a ball of radius 1 stretched by 6, 4 and 3 along x, y and z.
use crate::bench::cut;
use crate::shared::geometry::ellipsoid;
use crate::Refusal;
use solidcore_ops::{make_box, Body};

pub fn build() -> Result<Body, Refusal> {
    let whole = ellipsoid(6.0, 4.0, 3.0)?;
    let below = make_box([-10.0, -10.0, -10.0], [20.0, 20.0, 10.0])?;
    cut(whole, below)
}
