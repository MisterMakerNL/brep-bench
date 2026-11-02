//! A ball of radius 5 about the origin, where brepkit's sphere is centred already. The segment
//! count only sets how finely the kernel meshes it; the face is an exact sphere.
use brepkit_operations::primitives::make_sphere;
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

pub fn build(topo: &mut Topology) -> Result<SolidId, OperationsError> {
    make_sphere(topo, 5.0, 64)
}
