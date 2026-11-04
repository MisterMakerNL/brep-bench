//! Two cylinders of radius 5 and length 30 crossing at right angles, one along the x axis from
//! x = -15 to 15 and one along the y axis from y = -15 to 15, united.
use crate::bench::fuse;
use crate::shared::boolean::{cylinder_along_x, cylinder_along_y};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let along_x = cylinder_along_x([-15.0, 0.0, 0.0], 5.0, 30.0)?;
    let along_y = cylinder_along_y([0.0, -15.0, 0.0], 5.0, 30.0)?;
    fuse(along_x, along_y)
}
