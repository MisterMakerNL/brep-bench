//! A cylinder of radius 5 on the z axis from z = 0 to 10, bored along its axis by a hole of
//! radius 3: a cylinder from z = -1 to z = 11 cut from it.
use crate::bench::cut;
use crate::Refusal;
use solidcore_ops::{make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let rod = make_cylinder([0.0, 0.0, 0.0], 5.0, 10.0)?;
    let bore = make_cylinder([0.0, 0.0, -1.0], 3.0, 12.0)?;
    cut(rod, bore)
}
