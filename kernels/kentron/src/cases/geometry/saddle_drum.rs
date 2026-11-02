//! A round drum of radius 8 about the z axis, its floor at z = 0 and its top the saddle
//! z = 10 + xy/8, made as the bilinear B-spline patch on (−10, −10, 22.5), (10, −10, −2.5),
//! (−10, 10, −2.5), (10, 10, 22.5). The patch's face is swept straight down past the floor, and
//! the column's common with the drum's cylinder (from the floor up past the saddle) trims it.
use crate::shared::geometry::saddle;
use kentron::boolean;
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let column = saddle(m)?;
    let drum = Cylinder::new(8.0, 30.0).build(m)?;
    boolean::intersection(m, column, drum)
}
