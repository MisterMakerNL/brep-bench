//! The upper half of the sphere of radius 5 about the origin as a sheet, thickened by 1 to each
//! side. The sheet is one face on the sphere bounded by its equator, its normal pointing out of
//! the sphere.
//!
//! brepkit's thicken grows a face to one side only, so the sheet is thickened by 1 along its
//! normal and, again, by 1 against it, and the two shells fused along the sheet between them.
use brepkit_operations::thicken::thicken;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::fuse;
use crate::solids::offset::sphere_cap;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let sheet = sphere_cap(topo, 5.0, 0.0)?;
    let outer = thicken(topo, sheet, 1.0)?;
    let inner = thicken(topo, sheet, -1.0)?;
    Ok(fuse(topo, outer, inner)?)
}
