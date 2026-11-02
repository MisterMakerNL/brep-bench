//! A disc of radius 20 on the z axis from z = −5 to 0, with six holes of radius 2 cut one after
//! another. Each drill is the first (axis at (12, 0), running from z = −6 to 1, past both faces)
//! turned about the z axis by a multiple of 60°.
use brepkit_math::mat::Mat4;
use brepkit_operations::primitives::make_cylinder;
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::basic::upright_cylinder;
use crate::solids::{cut, placed};

pub fn build(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    let mut disc = upright_cylinder(topo, 20.0, 0.0, 0.0, -5.0, 0.0)?;
    for i in 0..6 {
        let drill = make_cylinder(topo, 2.0, 7.0)?;
        let turn = Mat4::rotation_z(f64::from(i * 60).to_radians());
        let drill = placed(topo, drill, turn * Mat4::translation(12.0, 0.0, -6.0))?;
        disc = cut(topo, disc, drill)?;
    }
    Ok(disc)
}
