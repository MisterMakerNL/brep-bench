//! A tooth space cut by a rack tooth rolled round a gear blank. brepkit has no sweep of a solid,
//! and none along a rolling motion; the space's flanks are the envelope of the tooth's flanks
//! over the motion (involutes of the base circle) and its root the curve its tip corners trace,
//! which brepkit has no operation to compute and no involute curve to build them from.
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::Refusal;

pub fn build(_topo: &mut Topology) -> Result<SolidId, Refusal> {
    Err(
        "brepkit has no sweep of a solid along a rolling motion, and the tooth space's flanks are \
         the envelope of the rack tooth's (involutes), which brepkit cannot compute"
            .into(),
    )
}
