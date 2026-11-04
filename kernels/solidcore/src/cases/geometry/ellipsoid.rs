//! The solid ellipsoid x²/36 + y²/16 + z²/9 ≤ 1, centred on the origin: solidcore has no
//! ellipsoid, so a ball of radius 1 stretched by 6, 4 and 3 along x, y and z.
use crate::shared::geometry::ellipsoid;
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    ellipsoid(6.0, 4.0, 3.0)
}
