//! The whole cylinder of radius 5 about the z axis from z = 0 to 10 as a sheet, thickened by 1
//! away from the axis. The sheet is the side of brepkit's cylinder, one face closing round on
//! itself, its normal pointing away from the axis.
use brepkit_operations::primitives::make_cylinder;
use brepkit_operations::thicken::thicken;
use brepkit_topology::face::FaceSurface;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::offset::face_on;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let cylinder = make_cylinder(topo, 5.0, 10.0)?;
    let sheet = face_on(topo, cylinder, |s| matches!(s, FaceSurface::Cylinder(_)))?;
    Ok(thicken(topo, sheet, 1.0)?)
}
