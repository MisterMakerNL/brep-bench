//! Two cylinders of radius 5 on the z axis, from z = 0 to 10 and from z = 6 to 16, united: their
//! walls coincide between z = 6 and 10. The second is turned a quarter turn about z, so that the
//! seams solidcore puts on each cylinder's +x side are not in the same place.
use crate::bench::{fuse, rotated};
use crate::Refusal;
use solidcore_ops::{make_cylinder, Body};

pub fn build() -> Result<Body, Refusal> {
    let first = make_cylinder([0.0, 0.0, 0.0], 5.0, 10.0)?;
    let second = make_cylinder([0.0, 0.0, 6.0], 5.0, 10.0)?;
    let second = rotated(second, [0.0; 3], [0.0, 0.0, 1.0], 90.0)?;
    fuse(first, second)
}
