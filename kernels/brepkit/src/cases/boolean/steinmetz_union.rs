//! Two cylinders of radius 5 and length 30 crossing at right angles, one along the x axis from
//! x = −15 to 15 and one along the y axis from y = −15 to 15: their union. Each is brepkit's
//! cylinder (on +z from the origin) turned onto its axis and moved to start at −15.
use brepkit_math::mat::Mat4;
use brepkit_operations::primitives::make_cylinder;
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::{fuse, placed};

pub fn build(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    let along_x = make_cylinder(topo, 5.0, 30.0)?;
    let to_x = Mat4::translation(-15.0, 0.0, 0.0) * Mat4::rotation_y(90f64.to_radians());
    let along_x = placed(topo, along_x, to_x)?;
    let along_y = make_cylinder(topo, 5.0, 30.0)?;
    let to_y = Mat4::translation(0.0, -15.0, 0.0) * Mat4::rotation_x(-90f64.to_radians());
    let along_y = placed(topo, along_y, to_y)?;
    fuse(topo, along_x, along_y)
}
