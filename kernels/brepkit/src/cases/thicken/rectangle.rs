//! The rectangle in the xy plane from the origin to (10, 8, 0), a sheet whose corners run
//! counter-clockwise seen from +z so its normal is +z, thickened by 2 along that normal.
use brepkit_operations::thicken::thicken;
use brepkit_topology::builder::make_planar_face;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::p;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let corners = [
        p(0.0, 0.0, 0.0),
        p(10.0, 0.0, 0.0),
        p(10.0, 8.0, 0.0),
        p(0.0, 8.0, 0.0),
    ];
    let sheet = make_planar_face(topo, &corners, 1e-7)?;
    Ok(thicken(topo, sheet, 2.0)?)
}
