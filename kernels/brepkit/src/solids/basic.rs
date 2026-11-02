//! Helpers for the primitives and boolean cases: a block given by two opposite corners, and a
//! cylinder standing on the z axis's direction at a given place. Both are brepkit's own box and
//! cylinder (anchored at the origin) moved into place.
use brepkit_operations::primitives::{make_box, make_cylinder};
use brepkit_operations::OperationsError;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use super::moved;

/// The block between the corners `lo` and `hi`.
pub fn block(topo: &mut Topology, lo: [f64; 3], hi: [f64; 3]) -> Result<SolidId, OperationsError> {
    let b = make_box(topo, hi[0] - lo[0], hi[1] - lo[1], hi[2] - lo[2])?;
    moved(topo, b, lo[0], lo[1], lo[2])
}

/// The cylinder of radius `r` whose axis is the line x = `x`, y = `y`, from z = `z0` to `z1`.
pub fn upright_cylinder(
    topo: &mut Topology,
    r: f64,
    x: f64,
    y: f64,
    z0: f64,
    z1: f64,
) -> Result<SolidId, OperationsError> {
    let c = make_cylinder(topo, r, z1 - z0)?;
    moved(topo, c, x, y, z0)
}
