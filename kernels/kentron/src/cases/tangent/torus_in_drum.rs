//! A solid torus about the z axis, tube radius 3 about a middle circle of radius 10 in the xy
//! plane, united with a drum, a cylinder of radius 13 on the z axis from z = −1 to z = 1. The
//! torus touches the drum's wall from inside along its outer equator; half of its tube stands out
//! of each of the drum's faces.
use crate::bench::{frame, p};
use kentron::boolean;
use kentron::math::Direction;
use kentron::primitives::{Cylinder, Torus};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let torus = Torus::new(10.0, 3.0).build(m)?;
    let drum = Cylinder::new(13.0, 2.0).with_frame(frame(p(0.0, 0.0, -1.0), Direction::Z)).build(m)?;
    boolean::union(m, torus, drum)
}
