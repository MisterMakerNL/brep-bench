//! The cap of the sphere of radius 10 about the origin where z ≥ 6 as a sheet, thickened by 2
//! away from the centre. The sheet is one face on the sphere bounded by the circle of radius 8 at
//! z = 6, its normal pointing out of the sphere.
use brepkit_operations::thicken::thicken;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::offset::sphere_cap;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let sheet = sphere_cap(topo, 10.0, 6.0)?;
    Ok(thicken(topo, sheet, 2.0)?)
}
