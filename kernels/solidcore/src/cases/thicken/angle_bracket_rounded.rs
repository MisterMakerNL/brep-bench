//! A sheet of two flat faces meeting at a right angle along a crease, the floor from (0, 0, 0) to
//! (10, 8, 0) in the xy plane and the wall from (0, 0, 0) to (0, 8, 10) in the yz plane,
//! thickened by 1 to the outside of the crease (to -z and -x), the gap the moved faces leave at
//! the crease closed by the quarter cylinder of radius 1 about it.
use crate::bench::{edge_through, reloaded};
use crate::shared::thicken::bracket;
use crate::Refusal;
use solidcore_ops::{blend, thicken, Blend, Body, Corners};

pub fn build() -> Result<Body, Refusal> {
    let sheet = bracket()?;
    // ops's thicken has no rounded corners (see `shared::thicken`): the sheet thickened with its
    // moved faces extended to meet at (-1, y, -1), then that corner rounded to radius 1, the
    // thickness: the round is tangent to both moved faces, about the crease.
    thicken(&sheet, 1.0, false)?;
    let (angle, order) = reloaded(&sheet)?;
    let corner = edge_through(&angle, &order, [-1.0, 4.0, -1.0])?;
    blend(&angle, Blend::Round, &[corner], &[1.0], &[], Corners::Filled)?;
    Ok(angle)
}
