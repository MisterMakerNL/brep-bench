//! A block, the box from the origin to (40, 30, 10), the edge where its top meets its back,
//! y = 30, rounded to radius 3; then its top face moved up by 6 to z = 16, carrying the round
//! with it: the round stays of radius 3, tangent to the top and the back, and the back and the
//! ends run on up to meet it.
use crate::bench::{edge_through, reloaded};
use crate::shared::edit::move_faces;
use crate::Refusal;
use solidcore_ops::{blend, make_box, Blend, Body, Corners};

pub fn build() -> Result<Body, Refusal> {
    let (block, order) = reloaded(&make_box([0.0, 0.0, 0.0], [40.0, 30.0, 10.0])?)?;
    // the body read back from its record, copied: ops' face offset fails (status 1001) on the
    // round of a body read back and blended, and not on a copy of it, which keeps its order
    let block = block.copy()?;
    let edge = edge_through(&block, &order, [20.0, 30.0, 10.0])?;
    blend(&block, Blend::Round, &[edge], &[3.0], &[], Corners::Filled)?;
    // the top, which ops' face offset moves with the tangent round on it
    move_faces(&block, &[[20.0, 10.0, 10.0]], 6.0)?;
    Ok(block)
}
