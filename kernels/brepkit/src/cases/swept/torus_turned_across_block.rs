//! A curved block grooved by a leaning torus turned past it about the z axis. brepkit has no
//! sweep of a solid: revolve turns a face, not a solid. Since the torus's axis is skew to the
//! turn's, the groove's wall is the envelope of the torus over the turn, none of its own circles,
//! which brepkit has no operation to compute.
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::Refusal;

pub fn build(_topo: &mut Topology) -> Result<SolidId, Refusal> {
    Err(
        "brepkit has no sweep of a solid (revolve turns a face), and the leaning torus turned \
         about the z axis is bounded by its envelope, which brepkit cannot compute"
            .into(),
    )
}
