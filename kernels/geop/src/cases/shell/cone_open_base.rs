//! A solid cone on the z axis, its base the disc of radius 6 at z = 0 and its apex at (0, 0, 8),
//! shelled inward to a wall 1 thick, open at its base. geop has no cone: it is turned from its
//! half section, a right triangle, about the z axis, then shelled by geop's shell, the base
//! picked by a point on it; the apex moves down the axis to the inner cone's apex.
use crate::shared::hollow::{at, shell, turned};
use crate::{Bench, Refusal};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    turned(b, "cone", |d| {
        let (o, r, apex) = (d.point(at(0.0, 0.0)), d.point(at(6.0, 0.0)), d.point(at(0.0, 8.0)));
        d.line(o, r);
        d.line(r, apex);
        d.line(apex, o)
    })?;
    shell(b, "cup", &[[0.0, 0.0, 0.0]], 1.0, false)
}
