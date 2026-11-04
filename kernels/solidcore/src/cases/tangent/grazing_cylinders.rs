//! Two cylinders of radius 5 and length 30 crossing at right angles: one along the x axis from
//! x = -15 to 15, one parallel to the y axis from y = -15 to 15 on the line x = 0, z = 9.99. The
//! solid common to both, a cushion 0.01 thick.
use crate::bench::common;
use crate::shared::boolean::{cylinder_along_x, cylinder_along_y};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let along_x = cylinder_along_x([-15.0, 0.0, 0.0], 5.0, 30.0)?;
    let along_y = cylinder_along_y([0.0, -15.0, 9.99], 5.0, 30.0)?;
    common(along_x, along_y)
}
