//! A hollow square tube tapering along the y axis: the square 24 × 24 at y = 0 lofted to the
//! square 12 × 12 at y = 40 (each corner running straight to its fellow, every long face a flat
//! trapezoid), less the hole, the square 20 × 20 lofted likewise to 8 × 8. brepkit's loft ignores
//! a profile's holes, so the hole is lofted on its own and cut out.
use brepkit_operations::loft::loft;
use brepkit_topology::face::FaceId;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::sweep::polygon;
use crate::solids::{cut, p};
use crate::Refusal;

/// The square of half-width `h` in the plane at `y`, centred on the y axis.
fn square(topo: &mut Topology, h: f64, y: f64) -> Result<FaceId, Refusal> {
    Ok(polygon(
        topo,
        &[p(-h, y, -h), p(h, y, -h), p(h, y, h), p(-h, y, h)],
    )?)
}

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let (foot, top) = (square(topo, 12.0, 0.0)?, square(topo, 6.0, 40.0)?);
    let outside = loft(topo, &[foot, top])?;
    let (foot, top) = (square(topo, 10.0, 0.0)?, square(topo, 4.0, 40.0)?);
    let hole = loft(topo, &[foot, top])?;
    Ok(cut(topo, outside, hole)?)
}
