//! A wheel cut by a plus-shaped tool turning in mesh with it. brepkit sweeps a face along a path
//! (sweep, pipe, revolve), but has no sweep of a solid, and none along a motion that turns the
//! tool while the wheel turns too. What the tool passes through, seen from the wheel, is bounded
//! by the envelope of its faces over the motion, which brepkit has no operation to compute.
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::Refusal;

pub fn build(_topo: &mut Topology) -> Result<SolidId, Refusal> {
    Err(
        "brepkit has no sweep of a solid along a motion, and the space the meshing plus cuts from \
         the wheel is bounded by the envelope of its faces, which brepkit cannot compute"
            .into(),
    )
}
