//! A napkin ring, the ball of radius 5 about the origin with a hole of radius 3 bored through it
//! along z, grown by 1: every point within 1 of it. The sphere moves out to radius 6, the bore
//! narrows to radius 2, and each rim becomes a piece of a torus's tube of radius 1 about the
//! rim's circle.
//!
//! geop has no ball: it is turned from a half disc about the z axis, and the bore cut by a
//! circle extruded past it both ways. geop has no offset of a solid that rounds where moved
//! faces part: its face offset moves the faces and meets them sharp. So the ring's faces are
//! offset 1 out, to the sphere of radius 6 and the bore of radius 2, and its two rims then
//! rounded to radius 1: each round is tangent to both moved faces, so it is the tube of radius
//! 1 about the ring's own rim.
use crate::shared::hollow::{at, offset_faces, round, turned, whole_edges};
use crate::shared::{Drawing, extrude_symmetric, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_fillet::FilletArgs;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let ball = turned(b, "ball", |d| {
        let (bottom, top) = (d.point(at(0.0, -5.0)), d.point(at(0.0, 5.0)));
        d.arc(top, bottom, 180.0);
        d.line(bottom, top)
    })?;
    let mut bore = Drawing::new();
    bore.circle([0.0, 0.0], 3.0);
    sketch(b, "bore", xy(), bore)?;
    extrude_symmetric(b, "hole", "bore", 20.0, Combine::cut(ball))?;
    let mut faces = round(5.0, 0.0);
    faces.extend(round(3.0, 0.0));
    offset_faces(b, "grow", &faces, 1.0)?;
    // The rims, where the sphere of radius 6 meets the bore of radius 2: at z = ±√32.
    let mut rims = round(2.0, 32f64.sqrt());
    rims.extend(round(2.0, -(32f64.sqrt())));
    let rims = whole_edges(b, &rims)?;
    b.step("rounds", FilletArgs::constant(rims, 1.0))
}
