//! A torus tumbled through space by two turns at once. brepkit has no sweep of a solid, and none
//! along a motion compounded of two turns; the solid's boundary is the envelope of the torus over
//! the motion, of no simpler kind, which brepkit has no operation to compute.
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::Refusal;

pub fn build(_topo: &mut Topology) -> Result<SolidId, Refusal> {
    Err(
        "brepkit has no sweep of a solid along a motion, and the tumbling torus's sweep is \
         bounded by its envelope, which brepkit cannot compute"
            .into(),
    )
}
