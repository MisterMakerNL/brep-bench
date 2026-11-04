//! The block from the origin to (20, 20, 10) with a hole of radius 3 bored through it along z
//! about x = 15.5, y = 10, shelled inward to a wall 1 thick, open at its top: the tube round the
//! bore runs into the wall x = 19 and merges with it. The block is extruded from its rectangle,
//! the hole cut by a circle extruded past both faces, and the solid shelled by geop's shell, the
//! top picked by a point on it.
use crate::shared::hollow::{block, shell};
use crate::shared::{Drawing, extrude_symmetric, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let body = block(b, "block", [0.0, 0.0], [20.0, 20.0], 10.0)?;
    // The bore: a circle of radius 3 extruded 30 each way from the xy plane, past both faces.
    let mut bore = Drawing::new();
    bore.circle([15.5, 10.0], 3.0);
    sketch(b, "bore", xy(), bore)?;
    extrude_symmetric(b, "hole", "bore", 60.0, Combine::cut(body))?;
    shell(b, "cup", &[[5.0, 10.0, 10.0]], 1.0, false)
}
