//! A block from (−10, −10, 0) to (10, 10, 10) united with a cone on the z axis standing on its tip
//! at the origin, the middle of the block's bottom face, its base the disc of radius 5 at z = 15.
//! brepkit's cone runs from its bottom radius at z = 0 to its top radius at its height, so a
//! bottom radius of 0 puts the apex at the origin.
use brepkit_operations::primitives::make_cone;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::contact::block;
use crate::solids::fuse;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let slab = block(topo, (-10.0, -10.0, 0.0), (10.0, 10.0, 10.0))?;
    let cone = make_cone(topo, 0.0, 5.0, 15.0)?;
    Ok(fuse(topo, slab, cone)?)
}
