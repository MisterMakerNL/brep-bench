//! The half of the cylinder of radius 5 about the z axis where y ≥ 0, from z = 0 to 10, as a
//! sheet thickened by 2 toward the axis. The sheet is the curved face of brepkit's cylinder
//! cut in half by a box (the half-cylinder solid's curved face, its normal pointing away from the
//! axis), thickened against that normal.
use brepkit_operations::thicken::thicken;
use brepkit_topology::face::FaceSurface;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::offset::{face_on, half_cylinder};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let half = half_cylinder(topo, 5.0, 10.0)?;
    let sheet = face_on(topo, half, |s| matches!(s, FaceSurface::Cylinder(_)))?;
    Ok(thicken(topo, sheet, -2.0)?)
}
