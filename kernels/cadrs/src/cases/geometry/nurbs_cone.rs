//! A solid cone standing on the disk of radius 5 about the z axis in the plane z = 0, its apex at
//! (0, 0, 10), whose side is one rational B-spline surface with a pole at the apex: rational
//! quadratic around the axis and linear up it, a 9 × 2 net whose first row is the base circle as
//! a rational quadratic (the nine points 5cᵢ, weights 1, √2/2, …, knots
//! (0, 0, 0, ¼, ¼, ½, ½, ¾, ¾, 1, 1, 1)) and whose second row is nine copies of the apex; closed
//! by a flat disk at its base.
//!
//! cadrs makes no face from a bare surface (its `face_from_surface` takes a face of a body), so
//! the side is made by the operation that makes it: a loft of two sections, the circle of radius
//! 5 sketched on the Top plane and the point (0, 0, 10). cadrs skins it with one ruled B-spline
//! face from the circle, which it holds as that rational quadratic of nine points, to the apex: the
//! net above, its whole edge v = 1 collapsed to the apex. The loft's cap at the circle is the
//! base's disk. No cone surface is used.
use crate::bench::{p3, round, sketch, xy};
use crate::Refusal;
use cadrs_kernel::{BodyId, BodyKind, DefaultKernel, Kernel, LoftEnd, LoftSection, LoftSpec};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let base = sketch(xy(0.0), vec![round((0.0, 0.0), 5.0)]);
    let spec = LoftSpec {
        body: BodyKind::Solid,
        sections: vec![LoftSection::Profile(base), LoftSection::Point(p3(0.0, 0.0, 10.0))],
        start: LoftEnd::default(),
        end: LoftEnd::default(),
        source: 1,
    };
    let r = k.loft_with(&spec)?;
    r.bodies.first().copied().ok_or_else(|| Refusal::from("the loft made no body"))
}
