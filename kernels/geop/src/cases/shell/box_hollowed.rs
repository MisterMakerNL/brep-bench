//! The box from the origin to (10, 8, 6), hollowed to a wall 1 thick with no opening: the box
//! less the box from (1, 1, 1) to (9, 7, 5), its cavity a second shell. The box is extruded from
//! its rectangle and shelled by geop's shell with no face picked, which closes it all round.
use crate::shared::hollow::{block, shell};
use crate::{Bench, Refusal};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    block(b, "box", [0.0, 0.0], [10.0, 8.0], 6.0)?;
    shell(b, "hollow", &[], 1.0, false)
}
