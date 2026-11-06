//! The box from the origin to (20, 12, 10), its four sides drafted 5° for the pull direction +z
//! about the neutral plane z = 5, halfway up: each side turns about the line where it crosses z =
//! 5, leaning inward as it rises.
use crate::solids::draft::faces_at;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{draft_faces, make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let solid = make_box_brep(p(0.0, 0.0, 0.0), 20.0, 12.0, 10.0)?;
    // Each side by a point in its middle.
    let sides = faces_at(
        &solid,
        &[p(10.0, 0.0, 5.0), p(20.0, 6.0, 5.0), p(10.0, 12.0, 5.0), p(0.0, 6.0, 5.0)],
    )?;
    Ok(draft_faces(&solid, &sides, p(0.0, 0.0, 1.0), p(0.0, 0.0, 5.0), 5f64.to_radians())?)
}
