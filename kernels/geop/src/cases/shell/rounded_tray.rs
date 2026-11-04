//! The box from the origin to (20, 14, 10), its four vertical edges rounded to radius 2,
//! shelled inward to a wall 1 thick, open at its top: a tray whose inside corners are rounds of
//! radius 1, concentric with the outside ones. The box is extruded from its rectangle, its
//! vertical edges filleted (each picked by a point on it), and shelled by geop's shell, the top
//! picked by a point on it.
use crate::shared::hollow::{block, shell};
use crate::{Bench, Refusal};
use geop_ops_fillet::FilletArgs;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    block(b, "plate", [0.0, 0.0], [20.0, 14.0], 10.0)?;
    let corners =
        b.edges(&[[0.0, 0.0, 5.0], [20.0, 0.0, 5.0], [20.0, 14.0, 5.0], [0.0, 14.0, 5.0]])?;
    b.step("rounds", FilletArgs::constant(corners, 2.0))?;
    shell(b, "tray", &[[10.0, 7.0, 10.0]], 1.0, false)
}
