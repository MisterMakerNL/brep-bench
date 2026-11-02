//! The inner half of the torus about the z axis whose tube has radius 2 about a middle circle of
//! radius 6 (the part of its surface no farther than 6 from the axis) as a sheet, thickened by 1
//! away from the middle circle, toward the axis. The sheet is the toroidal face of brepkit's torus
//! cut to the cylinder of radius 6 (their common part), its normal pointing out of the tube,
//! thickened along that normal.
use brepkit_operations::primitives::{make_cylinder, make_torus};
use brepkit_operations::thicken::thicken;
use brepkit_topology::face::FaceSurface;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::offset::face_on;
use crate::solids::{common, moved};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let torus = make_torus(topo, 6.0, 2.0, 64)?;
    // the cylinder runs past the tube's top and bottom
    let core = make_cylinder(topo, 6.0, 6.0)?;
    let core = moved(topo, core, 0.0, 0.0, -3.0)?;
    let inner = common(topo, torus, core)?;
    let sheet = face_on(topo, inner, |s| matches!(s, FaceSurface::Torus(_)))?;
    Ok(thicken(topo, sheet, 1.0)?)
}
