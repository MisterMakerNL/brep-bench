//! The bent sheet of `thicken/bent-flange` (a floor along z = 0 from x = −10 to 0, a bend round
//! the quarter circle of radius 2 about (0, ·, 2) to (2, ·, 2), and a wall up x = 2 to z = 12,
//! from y = 0 to 8), thickened by 3 to the inside of the bend, past the bend's radius.
//!
//! brepkit's thicken takes one face, so, as in `thicken/bent-flange`, each of the sheet's three
//! faces is thickened by 3 on its own, against its normal toward the bend's axis, and the pieces
//! are fused. The faces are those of the solid the sheet bounds with its profile closed across
//! the top and back (`bent_block`), whose outward normals point away from the bend's axis.
use brepkit_operations::thicken::thicken;
use brepkit_topology::solid::SolidId;
use brepkit_topology::Topology;

use crate::solids::fuse;
use crate::solids::offset::{bent_block, bent_sheet};
use crate::Refusal;

pub fn build(topo: &mut Topology) -> Result<SolidId, Refusal> {
    let block = bent_block(topo)?;
    let mut pieces = Vec::new();
    for face in bent_sheet(topo, block)? {
        pieces.push(thicken(topo, face, -3.0)?);
    }
    let mut bracket = pieces[0];
    for &piece in &pieces[1..] {
        bracket = fuse(topo, bracket, piece)?;
    }
    Ok(bracket)
}
