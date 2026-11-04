//! The plate from (−10, −10, 0) to (10, 10, 4) with a hole of radius 3 drilled through it along
//! the z axis, grown by 1: every point within 1 of it. The faces move 1 outward (the hole's
//! wall to radius 2), each outer edge becomes a quarter cylinder, each corner an eighth of a
//! ball, and each of the hole's rims the inner quarter of a torus's tube of radius 1 about it.
//!
//! geop has no offset of a solid that rounds where moved faces part: its face offset moves the
//! faces and meets them sharp. So the drilled plate's faces are offset 1 out, to the plate from
//! (−11, −11, −1) to (11, 11, 5) with a hole of radius 2, and its edges then rounded to radius
//! 1: each round is tangent to two moved faces, so it lies about the plate's own edge or rim.
use crate::shared::hollow::{block, offset_faces, round, whole_edges};
use crate::shared::{Drawing, extrude_symmetric, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_fillet::FilletArgs;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let plate = block(b, "plate", [-10.0, -10.0], [10.0, 10.0], 4.0)?;
    let mut bore = Drawing::new();
    bore.circle([0.0, 0.0], 3.0);
    sketch(b, "bore", xy(), bore)?;
    extrude_symmetric(b, "hole", "bore", 20.0, Combine::cut(plate))?;
    let mut faces = vec![
        [5.0, 5.0, 0.0],
        [5.0, 5.0, 4.0],
        [0.0, -10.0, 2.0],
        [10.0, 0.0, 2.0],
        [0.0, 10.0, 2.0],
        [-10.0, 0.0, 2.0],
    ];
    faces.extend(round(3.0, 2.0));
    offset_faces(b, "grow", &faces, 1.0)?;
    let mut edges = Vec::new();
    for z in [-1.0, 5.0] {
        edges.extend([[0.0, -11.0, z], [11.0, 0.0, z], [0.0, 11.0, z], [-11.0, 0.0, z]]);
        edges.extend(round(2.0, z));
    }
    edges.extend([[-11.0, -11.0, 2.0], [11.0, -11.0, 2.0], [11.0, 11.0, 2.0], [-11.0, 11.0, 2.0]]);
    let edges = whole_edges(b, &edges)?;
    b.step("rounds", FilletArgs::constant(edges, 1.0))
}
