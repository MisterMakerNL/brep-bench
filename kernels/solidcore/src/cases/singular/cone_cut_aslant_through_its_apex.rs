//! A solid cone on the z axis, its base the disc of radius 5 on the xy plane and its apex at
//! (0, 0, 10), less the side beyond the plane 4x + z = 10, which passes through the apex and
//! meets the base plane in the line x = 2.5: the half-space is the block beyond x = 2.5 tilted
//! about the line through (2.5, 0, 0) along y until its face's normal is (4, 0, 1)/√17.
use crate::bench::cut;
use crate::shared::singular::beyond;
use crate::Refusal;
use solidcore_ops::{make_cone, Body};

pub fn build() -> Result<Body, Refusal> {
    let cone = make_cone([0.0, 0.0, 0.0], 5.0, 0.0, 10.0)?;
    // turning +x about y by -atan(1/4) takes it to (4, 0, 1)/√17
    cut(cone, beyond(2.5, -(0.25f64.atan().to_degrees()))?)
}
