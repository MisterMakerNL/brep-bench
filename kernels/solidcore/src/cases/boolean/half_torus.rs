//! A torus about the z axis, centred on the origin, of radii 10 (its tube's centre circle) and
//! 2 (its tube), with everything where x < 0 cut away: a block over x < 0 reaching past it.
use crate::bench::cut;
use crate::Refusal;
use solidcore_ops::{make_box, make_torus, Body};

pub fn build() -> Result<Body, Refusal> {
    let torus = make_torus([0.0, 0.0, 0.0], 10.0, 2.0)?;
    let negative_x = make_box([-15.0, -15.0, -5.0], [15.0, 30.0, 10.0])?;
    cut(torus, negative_x)
}
