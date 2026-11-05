//! The upper half of the sphere of radius 5 about the origin, a sheet of one face, thickened by
//! 1 to each side: the dome between the spheres of radius 4 and 6 where z ≥ 0.
use crate::shared::thicken::{hemisphere, thickened};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    // 2 in all, moved back by half first: 1 to each side
    thickened(hemisphere()?, 2.0, true)
}
