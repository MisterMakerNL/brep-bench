//! The box from the origin to (10, 8, 6), shelled inward to a wall 1 thick, open at its top,
//! z = 6: the textbook shell. The box is extruded from its rectangle and shelled by geop's shell,
//! the top picked by a point on it.
use crate::shared::hollow::{block, shell};
use crate::{Bench, Refusal};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    block(b, "box", [0.0, 0.0], [10.0, 8.0], 6.0)?;
    shell(b, "cup", &[[5.0, 4.0, 6.0]], 1.0, false)
}
