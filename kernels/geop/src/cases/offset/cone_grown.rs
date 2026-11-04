//! A cone on the z axis, its base the disc of radius 3 at z = 0 and its apex at (0, 0, 4),
//! grown by 1: every point within 1 of it. The base moves down 1, the side out 1, the rim
//! becomes a piece of a torus's tube and the apex a cap of a ball of radius 1 about it.
//!
//! geop has no offset of a solid that rounds where moved faces part: its face offset moves the
//! faces and meets them sharp. A rim can be rounded after by a fillet, but the apex is a point,
//! not an edge: no fillet puts the cap of a ball round it, and the sharp offset's apex, 5/3
//! higher, would be left. So it is refused.
use crate::{Bench, Refusal};

pub fn build(_: &mut Bench) -> Result<(), Refusal> {
    Err("geop has no rounded offset of a solid: its face offset meets moved faces sharp, and the \
         cone's apex, a point and not an edge, cannot be rounded into the cap of a ball after"
        .into())
}
