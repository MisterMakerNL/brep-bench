//! The upper half of the sphere of radius 5 about the origin as a sheet, thickened by 5 toward the
//! centre. The sheet is one face on the sphere bounded by its equator, its normal pointing out of
//! the sphere, thickened against that normal.
use brepkit_operations::thicken::thicken;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::offset::sphere_cap;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let sheet = sphere_cap(topo, 5.0, 0.0)?;
    Ok(thicken(topo, sheet, -5.0)?)
}
