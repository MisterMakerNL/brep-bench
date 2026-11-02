//! A solid ellipsoid centred on the origin, its semi-axes 6 along x, 4 along y and 3 along z:
//! every point with x²/36 + y²/16 + z²/9 ≤ 1. kentron has no ellipsoid, so it is the ball of
//! radius 1 stretched by 6, 4 and 3 along x, y and z (as the description allows), a general
//! affine map that `modify::transform_affine` carries out exactly, as a rational B-spline.
use kentron::math::{AffineTransform, Matrix3, Vector};
use kentron::modify;
use kentron::primitives::Sphere;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let ball = Sphere::new(1.0).build(m)?;
    let stretch = Matrix3::from_rows([[6.0, 0.0, 0.0], [0.0, 4.0, 0.0], [0.0, 0.0, 3.0]]);
    modify::transform_affine(m, ball, &AffineTransform::new(stretch, Vector::ZERO))
}
