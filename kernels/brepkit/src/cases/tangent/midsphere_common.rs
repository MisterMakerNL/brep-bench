//! A cube of side 10 about the origin and the ball about the origin of radius 5√2, whose sphere
//! touches each of the cube's edges at its midpoint: their common part.
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::common;
use crate::solids::contact::{ball, block};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let cube = block(topo, (-5.0, -5.0, -5.0), (5.0, 5.0, 5.0))?;
    let midsphere = ball(topo, 5.0 * 2f64.sqrt(), (0.0, 0.0, 0.0))?;
    Ok(common(topo, cube, midsphere)?)
}
