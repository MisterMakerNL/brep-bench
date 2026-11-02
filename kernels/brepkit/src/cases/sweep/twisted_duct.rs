//! A duct from the square ABCD at y = 0 to the diamond A′B′C′D′ at y = 40: brepkit's loft of the
//! two, each corner joined straight to its fellow, so each long face is the ruled surface between
//! two neighbouring corner lines. Both sections are made from their corners in the same order,
//! A to D and A′ to D′, which is how the loft pairs them.
use brepkit_operations::loft::loft;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::p;
use crate::solids::sweep::polygon;
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let square = polygon(
        topo,
        &[
            p(10.0, 0.0, -10.0),
            p(-10.0, 0.0, -10.0),
            p(-10.0, 0.0, 10.0),
            p(10.0, 0.0, 10.0),
        ],
    )?;
    let diamond = polygon(
        topo,
        &[
            p(0.0, 40.0, -12.0),
            p(-12.0, 40.0, 0.0),
            p(0.0, 40.0, 12.0),
            p(12.0, 40.0, 0.0),
        ],
    )?;
    Ok(loft(topo, &[square, diamond])?)
}
