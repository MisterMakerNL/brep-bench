//! The lower half of the ball of radius 5 about the origin, shelled inward to a wall 1 thick,
//! open at its flat face: a bowl. geop has no ball: the half ball is turned from a quarter disc
//! about the z axis, then shelled by geop's shell, the flat face picked by a point on it.
use crate::shared::hollow::{at, shell, turned};
use crate::{Bench, Refusal};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    turned(b, "half_ball", |d| {
        let (o, r, pole) = (d.point(at(0.0, 0.0)), d.point(at(5.0, 0.0)), d.point(at(0.0, -5.0)));
        d.line(o, r);
        // From (5, 0) down round to the pole: clockwise in the xz plane, counter-clockwise in the
        // sketch's (x, −z).
        d.arc(r, pole, 90.0);
        d.line(pole, o)
    })?;
    shell(b, "bowl", &[[0.0, 0.0, 0.0]], 1.0, false)
}
