//! A ball of radius 5 centred on the origin, its poles on the z axis (solidcore's own), with
//! everything on the side x < 0 cut away: the cut runs through both poles.
use crate::bench::cut;
use crate::Refusal;
use solidcore_ops::{make_box, make_sphere, Body};

pub fn build() -> Result<Body, Refusal> {
    let ball = make_sphere([0.0, 0.0, 0.0], 5.0)?;
    let negative_x = make_box([-10.0, -10.0, -10.0], [10.0, 20.0, 20.0])?;
    cut(ball, negative_x)
}
