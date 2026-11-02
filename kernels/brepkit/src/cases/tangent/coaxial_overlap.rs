//! Two cylinders of radius 5 on the z axis, from z = 0 to 10 and from z = 6 to 16, united. brepkit's
//! cylinder has its seam on +x, so the second is turned a quarter turn about z before it is
//! raised, to keep the two seams apart where the walls coincide.
use brepkit_math::mat::Mat4;
use brepkit_operations::primitives::make_cylinder;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;
use std::f64::consts::FRAC_PI_2;

use crate::solids::{fuse, placed};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let lower = make_cylinder(topo, 5.0, 10.0)?;
    let upper = make_cylinder(topo, 5.0, 10.0)?;
    let upper = placed(
        topo,
        upper,
        Mat4::translation(0.0, 0.0, 6.0) * Mat4::rotation_z(FRAC_PI_2),
    )?;
    Ok(fuse(topo, lower, upper)?)
}
