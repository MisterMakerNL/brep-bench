//! A cylinder of radius 5 on the z axis from z = 0 to z = 10, grown by 1: every point within 1
//! of it. The side moves out to radius 6, the ends to z = −1 and z = 11, and each rim becomes the
//! outer quarter of a torus's tube of radius 1 about the rim's circle.
//!
//! geop has no offset of a solid that rounds where moved faces part: its face offset moves the
//! faces and meets them sharp. So the cylinder's faces are offset 1 out, to the cylinder of
//! radius 6 from z = −1 to z = 11, and its two rims then rounded to radius 1: each round is
//! tangent to the moved side and the moved end, so it is the tube of radius 1 about the
//! cylinder's own rim.
use crate::shared::hollow::{offset_faces, round, whole_edges};
use crate::shared::{Drawing, extrude, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_fillet::FilletArgs;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.circle([0.0, 0.0], 5.0);
    sketch(b, "base", xy(), base)?;
    extrude(b, "can", "base", 10.0, Combine::NewBody)?;
    let mut faces = round(5.0, 5.0);
    faces.extend([[0.0, 0.0, 0.0], [0.0, 0.0, 10.0]]);
    offset_faces(b, "grow", &faces, 1.0)?;
    let mut rims = round(6.0, -1.0);
    rims.extend(round(6.0, 11.0));
    let rims = whole_edges(b, &rims)?;
    b.step("rounds", FilletArgs::constant(rims, 1.0))
}
