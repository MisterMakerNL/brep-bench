//! A ball of radius 10 centred on the origin, and a cylinder of radius 5 on the line x = 5, y = 0
//! from z = -12 to z = 12, whose wall touches the sphere from inside at (10, 0, 0); they meet
//! in Viviani's curve. The ball's poles are on the y axis and its seam through (-10, 0, 0), the
//! cylinder's seam on the z axis.
//! The ball less the cylinder, pinched to the point where they touch.
use crate::bench::cut;
use crate::shared::tangent::{viviani_ball, viviani_cylinder};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    cut(viviani_ball()?, viviani_cylinder()?)
}
