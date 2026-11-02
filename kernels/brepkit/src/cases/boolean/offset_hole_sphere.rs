//! A ball of radius 20 about the origin, pierced parallel to the y axis by a hole of radius 6 on
//! the line x = 5, z = 0. The drill is brepkit's cylinder (on +z) turned −90° about x so that it
//! runs along +y, and moved so it runs from y = −25 to 25, past the ball at both ends.
use brepkit_math::mat::Mat4;
use brepkit_operations::primitives::{make_cylinder, make_sphere};
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::{cut, placed};

pub fn build(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    let ball = make_sphere(topo, 20.0, 64)?;
    let drill = make_cylinder(topo, 6.0, 50.0)?;
    let along_y = Mat4::translation(5.0, -25.0, 0.0) * Mat4::rotation_x(-90f64.to_radians());
    let drill = placed(topo, drill, along_y)?;
    cut(topo, ball, drill)
}
