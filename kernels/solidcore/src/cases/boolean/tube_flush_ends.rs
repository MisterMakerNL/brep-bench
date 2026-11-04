//! A cylinder of radius 5 on the z axis from z = 0 to 10, less a cylinder of radius 3 on the same
//! axis over exactly the same span: the cutter's ends lie in the first's.
use crate::bench::cut;
use crate::Refusal;
use solidcore_ops::{make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let rod = make_cylinder([0.0, 0.0, 0.0], 5.0, 10.0)?;
    let bore = make_cylinder([0.0, 0.0, 0.0], 3.0, 10.0)?;
    cut(rod, bore)
}
