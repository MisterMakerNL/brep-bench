//! A ball of radius 5 about the origin, hollowed to a wall 1 thick with no opening: the ball
//! less the ball of radius 4. geop has no ball: it is turned from a half disc about the z axis,
//! then shelled by geop's shell with no face picked, which closes it all round.
use crate::shared::hollow::{at, shell, turned};
use crate::{Bench, Refusal};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    turned(b, "ball", |d| {
        let (bottom, top) = (d.point(at(0.0, -5.0)), d.point(at(0.0, 5.0)));
        d.arc(top, bottom, 180.0);
        d.line(bottom, top)
    })?;
    shell(b, "hollow", &[], 1.0, false)
}
