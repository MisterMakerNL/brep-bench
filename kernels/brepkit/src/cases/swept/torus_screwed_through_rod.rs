//! A rod grooved by a torus carried along a screw. brepkit has no sweep of a solid: its helical
//! sweep carries a flat profile face along a helix. The groove's wall is the envelope of the
//! leaning torus over the screw motion, none of the torus's own circles, which brepkit has no
//! operation to compute.
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::Refusal;

pub fn build(_topo: &mut Topology) -> Result<SolidId, Refusal> {
    Err(
        "brepkit has no sweep of a solid (its helical sweep carries a flat face), and the groove \
         the torus screws is bounded by its envelope, which brepkit cannot compute"
            .into(),
    )
}
