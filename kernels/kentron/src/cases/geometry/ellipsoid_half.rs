//! The ellipsoid x²/36 + y²/16 + z²/9 ≤ 1 with everything below z = 0 cut away: a dome on the
//! flat ellipse x²/36 + y²/16 ≤ 1, rising to 3. kentron has no ellipsoid, so it is the upper half
//! of the ball of radius 1 (the sphere primitive between latitudes 0 and 90°, closed by its flat
//! equatorial disc) stretched by 6, 4 and 3 along x, y and z, a general affine map that
//! `modify::transform_affine` carries out exactly.
//!
//! (The whole stretched ball cut by a block on z = 0, `boolean::intersection`, gives an area
//! 1.3e-6 too large: the trimmed B-spline face's boundary is the boolean's approximation.)
use kentron::math::{AffineTransform, Matrix3, Vector};
use kentron::modify;
use kentron::primitives::Sphere;
use kentron::topology::{Model, Shape};
use std::f64::consts::FRAC_PI_2;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let dome = Sphere::new(1.0).with_latitudes(0.0, FRAC_PI_2).build(m)?;
    let stretch = Matrix3::from_rows([[6.0, 0.0, 0.0], [0.0, 4.0, 0.0], [0.0, 0.0, 3.0]]);
    modify::transform_affine(m, dome, &AffineTransform::new(stretch, Vector::ZERO))
}
