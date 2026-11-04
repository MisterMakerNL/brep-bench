//! The box from the origin to (10, 8, 6), shelled inward to a wall 1 thick, open at its top and
//! its front, y = 0, two faces sharing an edge. The box is extruded from its rectangle and
//! shelled by geop's shell, the two faces picked by a point on each.
use crate::shared::hollow::{block, shell};
use crate::{Bench, Refusal};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    block(b, "box", [0.0, 0.0], [10.0, 8.0], 6.0)?;
    shell(b, "scoop", &[[5.0, 4.0, 6.0], [5.0, 0.0, 3.0]], 1.0, false)
}
