//! The side of the cone frustum about the z axis whose radius runs from 6 at z = 0 to 3 at
//! z = 4 as a sheet, thickened by 1 along its normal away from the axis. The sheet is the side of
//! brepkit's frustum, its normal the frustum's outward one.
use brepkit_operations::primitives::make_cone;
use brepkit_operations::thicken::thicken;
use brepkit_topology::face::FaceSurface;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::offset::face_on;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let frustum = make_cone(topo, 6.0, 3.0, 4.0)?;
    let sheet = face_on(topo, frustum, |s| matches!(s, FaceSurface::Cone(_)))?;
    Ok(thicken(topo, sheet, 1.0)?)
}
