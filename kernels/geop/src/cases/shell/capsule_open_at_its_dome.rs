//! A capsule's end, a cylinder of radius 5 on the z axis from z = 0 to z = 10 capped by the half
//! ball of radius 5 about (0, 0, 10), shelled inward to a wall 1 thick, open at its hemisphere:
//! the side's inner wall runs on past z = 10 until it meets the sphere. geop has no ball: the
//! capsule is turned from its half section (a rectangle and a quarter circle) about the z axis,
//! then shelled by geop's shell, the hemisphere picked by a point on it.
use crate::shared::hollow::{at, shell, turned};
use crate::{Bench, Refusal};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    turned(b, "capsule", |d| {
        let (o, r, s, top) = (
            d.point(at(0.0, 0.0)),
            d.point(at(5.0, 0.0)),
            d.point(at(5.0, 10.0)),
            d.point(at(0.0, 15.0)),
        );
        d.line(o, r);
        d.line(r, s);
        // From the side's top over to the pole, a quarter turn about (0, 0, 10): clockwise in
        // the sketch's (x, −z).
        d.arc(s, top, -90.0);
        d.line(top, o)
    })?;
    shell(b, "cup", &[[0.0, 0.0, 15.0]], 1.0, false)
}
