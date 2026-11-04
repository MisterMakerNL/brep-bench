//! A cylinder of radius 5 along the x axis from x = -15 to 15, and one of radius 3 parallel to
//! the y axis from y = -15 to 15 on the line x = 0, z = 2, whose wall touches the first's from
//! inside at (0, 0, 5): the solid common to both.
use crate::bench::common;
use crate::shared::boolean::{cylinder_along_x, cylinder_along_y};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let main = cylinder_along_x([-15.0, 0.0, 0.0], 5.0, 30.0)?;
    let branch = cylinder_along_y([0.0, -15.0, 2.0], 3.0, 30.0)?;
    common(main, branch)
}
