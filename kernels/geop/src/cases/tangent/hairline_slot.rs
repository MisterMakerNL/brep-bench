//! A cube of side 10 between the origin and (10, 10, 10) with a saw cut a millionth wide down into
//! it from the top: the block between x = 5 and x = 5.000001, from z = 5 up through the top,
//! right through in y. The cube is a square on the xy plane extruded 10 up; the slot a rectangle
//! on the xz plane from (5, 5) to (5.000001, 11), past the top, extruded from y = −1 to y = 11,
//! past the front and back, and cut from it.
use crate::shared::booleans::extrude_between;
use crate::shared::{Drawing, extrude, made, sketch, xy, zx, zx_at};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([0.0, 0.0], [10.0, 10.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "cube", "base", 10.0, Combine::NewBody)?;
    let mut kerf = Drawing::new();
    kerf.rectangle(zx_at(5.0, 5.0), zx_at(5.000001, 11.0));
    sketch(b, "kerf", zx(), kerf)?;
    extrude_between(b, "slot", "kerf", -1.0, 11.0, Combine::cut(made("extrude", "cube")))
}
