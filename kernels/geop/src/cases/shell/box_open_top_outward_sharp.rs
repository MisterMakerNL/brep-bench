//! The box from the origin to (10, 8, 6), shelled outward to a wall 1 thick, open at its top,
//! its edges kept sharp: the box from (−1, −1, −1) to (11, 9, 6) less the box.
//!
//! geop's shell grows walls outward with their faces extended until they meet, but cannot open
//! such walls at a face (its own refusal: the top meets the sides at an inward corner of the
//! walls). So the box is shelled outward closed all round, a wall 1 thick on every face, and the
//! wall over the top, everything above the plane of the top, z = 6, is cut away: a rectangle
//! drawn on that plane (a datum offset from the xy plane) past the wall, extruded up through it.
use crate::shared::hollow::{block, level, shell};
use crate::shared::{Drawing, extrude, made, sketch};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    block(b, "box", [0.0, 0.0], [10.0, 8.0], 6.0)?;
    shell(b, "walls", &[], 1.0, true)?;
    let top = level(b, "top", 6.0)?;
    let mut lid = Drawing::new();
    lid.rectangle([-5.0, -5.0], [20.0, 20.0]);
    sketch(b, "lid", top, lid)?;
    extrude(b, "open", "lid", 5.0, Combine::cut(made("shell", "walls")))
}
