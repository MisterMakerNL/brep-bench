//! The block from the origin to (20, 20, 10) with a hole of radius 3 bored through it along z
//! about x = 10, y = 10, shelled inward to a wall 1 thick, open at the bore: a hollow box whose
//! cavity opens into the hole. The block is extruded from its rectangle, the hole cut by a circle
//! extruded past both faces, and the solid shelled by geop's shell, the bore picked by points
//! round it (geop keeps it in pieces).
use crate::shared::hollow::{block, shell};
use crate::shared::{Drawing, extrude_symmetric, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let body = block(b, "block", [0.0, 0.0], [20.0, 20.0], 10.0)?;
    // The bore: a circle of radius 3 extruded 30 each way from the xy plane, past both faces.
    let mut bore = Drawing::new();
    bore.circle([10.0, 10.0], 3.0);
    sketch(b, "bore", xy(), bore)?;
    extrude_symmetric(b, "hole", "bore", 60.0, Combine::cut(body))?;
    shell(
        b,
        "hollow",
        &[[13.0, 10.0, 5.0], [7.0, 10.0, 5.0], [10.0, 13.0, 5.0], [10.0, 7.0, 5.0]],
        1.0,
        false,
    )
}
