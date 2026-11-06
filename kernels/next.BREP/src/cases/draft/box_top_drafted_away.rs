//! The box from the origin to (30, 10, 10), its four sides drafted 30° for the pull direction +z
//! about the neutral plane z = 0, the floor: the long sides meet below the top, which the draft
//! consumes, leaving a hip roof. The kernel's draft takes a draft that consumes a face of a convex
//! body as the intersection of its faces' half-spaces, the drafted ones turned.
use crate::solids::draft::faces_at;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{draft_faces, make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let solid = make_box_brep(p(0.0, 0.0, 0.0), 30.0, 10.0, 10.0)?;
    let sides = faces_at(
        &solid,
        &[p(15.0, 0.0, 5.0), p(30.0, 5.0, 5.0), p(15.0, 10.0, 5.0), p(0.0, 5.0, 5.0)],
    )?;
    Ok(draft_faces(&solid, &sides, p(0.0, 0.0, 1.0), p(0.0, 0.0, 0.0), 30f64.to_radians())?)
}
