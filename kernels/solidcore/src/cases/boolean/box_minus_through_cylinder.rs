//! A cube of side 10 between the origin and (10, 10, 10), drilled through along z by a hole of
//! radius 3 centred at x = 5, y = 5: a cylinder from z = -1 to z = 11 cut from it.
use crate::bench::cut;
use crate::Refusal;
use solidcore_ops::{make_box, make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let cube = make_box([0.0, 0.0, 0.0], [10.0, 10.0, 10.0])?;
    let drill = make_cylinder([5.0, 5.0, -1.0], 3.0, 12.0)?;
    cut(cube, drill)
}
