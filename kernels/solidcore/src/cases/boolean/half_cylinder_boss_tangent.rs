//! A cube of side 10 from the origin to (10, 10, 10), united with a cylinder of radius 5 and
//! height 10 on the xy plane, its axis at x = 10, y = 5: half of it inside the cube, its wall
//! touching the faces y = 0 and y = 10 along the cube's edges at x = 10.
use crate::bench::fuse;
use crate::Refusal;
use solidcore_ops::{make_box, make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let cube = make_box([0.0, 0.0, 0.0], [10.0, 10.0, 10.0])?;
    let boss = make_cylinder([10.0, 5.0, 0.0], 5.0, 10.0)?;
    fuse(cube, boss)
}
