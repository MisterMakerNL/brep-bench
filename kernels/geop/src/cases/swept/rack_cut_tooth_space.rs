//! A gear blank with one tooth space cut by a rack tooth rolled round it, as a gear is generated.
//!
//! geop sweeps a profile along a path, turning it with the path; it has no sweep of a solid
//! through a motion, and the space's flanks are the envelope of the tooth's flanks as the rack
//! rolls — involutes that would have to be computed here, outside geop.
use crate::{Bench, Refusal};

pub fn build(_: &mut Bench) -> Result<(), Refusal> {
    Err("geop has no sweep of a solid through a motion: the space the rolling rack tooth cuts is \
         bounded by the envelope of its flanks, which geop cannot build"
        .into())
}
