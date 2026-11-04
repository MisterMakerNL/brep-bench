//! The part of the ball of radius 5 about the origin below z = 3, shelled inward to a wall 1
//! thick, open at its flat face: the inner sphere meets the opening's plane aslant. geop has no
//! ball: it is turned from a half disc about the z axis and cut by a block above z = 3, then
//! shelled by geop's shell, the flat face picked by a point on it.
use crate::shared::hollow::{at, shell, turned};
use crate::shared::{Drawing, extrude_symmetric, made, sketch, zx};
use crate::{Bench, Refusal};
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    turned(b, "ball", |d| {
        let (bottom, top) = (d.point(at(0.0, -5.0)), d.point(at(0.0, 5.0)));
        d.arc(top, bottom, 180.0);
        d.line(bottom, top)
    })?;
    // Everything above z = 3 cut away: a rectangle on the xz plane reaching past the ball,
    // extruded through it both ways.
    let mut above = Drawing::new();
    above.rectangle(at(-6.0, 3.0), at(6.0, 6.0));
    sketch(b, "above", zx(), above)?;
    extrude_symmetric(b, "cap", "above", 12.0, Combine::cut(made("revolve", "ball")))?;
    shell(b, "bowl", &[[0.0, 0.0, 3.0]], 1.0, false)
}
