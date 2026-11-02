//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, with a cylinder of radius 3 on the
//! same axis, also exactly from z = 0 to z = 10, cut from it: the cutter's ends lie in the ends
//! of the first.
use kentron::boolean;
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let outer = Cylinder::new(5.0, 10.0).build(m)?;
    let inner = Cylinder::new(3.0, 10.0).build(m)?;
    boolean::difference(m, outer, inner)
}
