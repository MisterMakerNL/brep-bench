//! A drum, the cylinder of radius 10 on the z axis from z = 0 to z = 10, with a groove 1.6
//! wide and 3 deep round its side between z = 4.2 and z = 5.8, grown by 1: every point within 1
//! of it. The groove fills, and its outer rims become pieces of tori meeting in a shallow notch.
//!
//! geop has no offset of a solid that is every point within a distance of it: its face offset
//! moves the faces and meets them sharp, keeping every face, so it can neither fill the groove
//! whose moved walls cross nor round the groove's rims into the tori that make the notch. So it
//! is refused.
use crate::{Bench, Refusal};

pub fn build(_: &mut Bench) -> Result<(), Refusal> {
    Err("geop has no offset of a solid as every point within a distance of it: its face offset \
         keeps every face and meets them sharp, so it can neither fill the groove whose moved \
         walls cross nor make the rounds that form the notch"
        .into())
}
