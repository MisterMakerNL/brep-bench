//! The sphere of radius 5 about the origin as a closed sheet, thickened by 1 outward into the
//! hollow ball between radii 5 and 6.
//!
//! brepkit's thicken takes one face, and its sphere is two (its hemispheres), so the closed sheet
//! is thickened as the boundary of brepkit's ball by its shell grown outward with no opening: the
//! wall 1 thick outside the sphere, closed round the ball as a void.
use brepkit_operations::offset_v2::shell_v2;
use brepkit_operations::primitives::make_sphere;
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

pub fn build(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    let ball = make_sphere(topo, 5.0, 64)?;
    shell_v2(topo, ball, 1.0, &[])
}
