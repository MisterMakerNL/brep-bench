//! A drum, the cylinder of radius 8 on the z axis from z = 0 to z = 6, with a flange from
//! radius 8 out to 11 between z = 2.2 and z = 3.8, shrunk by 1: every point at least 1 from its
//! outside. The flange, thinner than 2, is gone, and where it met the drum two concave pieces of
//! tori meet in a low ridge.
//!
//! geop has no offset of a solid that is every point at least a distance from its outside: its
//! face offset moves the faces and meets them sharp, keeping every face, so it can neither drop
//! the flange whose moved faces cross nor round the concave edges where the flange met the drum
//! into the tori that make the ridge. So it is refused.
use crate::{Bench, Refusal};

pub fn build(_: &mut Bench) -> Result<(), Refusal> {
    Err("geop has no offset of a solid as every point at least a distance from its outside: its \
         face offset keeps every face and meets them sharp, so it can neither drop the flange \
         whose moved faces cross nor make the rounds that form the ridge"
        .into())
}
