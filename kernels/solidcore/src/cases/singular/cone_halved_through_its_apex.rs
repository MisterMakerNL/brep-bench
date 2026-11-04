//! A solid cone on the z axis, its base the disc of radius 5 on the xy plane and its apex at
//! (0, 0, 10), with everything on the side x < 0 cut away: the cut holds the axis and the apex.
use crate::bench::cut;
use crate::Refusal;
use solidcore_ops::{make_box, make_cone, Body};

pub fn build() -> Result<Body, Refusal> {
    let cone = make_cone([0.0, 0.0, 0.0], 5.0, 0.0, 10.0)?;
    let negative_x = make_box([-10.0, -10.0, -5.0], [10.0, 20.0, 20.0])?;
    cut(cone, negative_x)
}
