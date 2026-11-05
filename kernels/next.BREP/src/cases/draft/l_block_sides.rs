//! The L through (0, 0), (30, 0), (30, 10), (10, 10), (10, 25), (0, 25), extruded from z = 0 to
//! z = 12, its six sides drafted 6° for the pull direction +z about the neutral plane z = 0, the
//! floor: each side turns about its bottom edge, leaning inward as it rises, the two at the concave
//! edge through (10, 10) too.
use crate::solids::draft::faces_at;
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{draft_faces, extrude_profile_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let l = polygon(&[
        p(0.0, 0.0, 0.0), p(30.0, 0.0, 0.0), p(30.0, 10.0, 0.0),
        p(10.0, 10.0, 0.0), p(10.0, 25.0, 0.0), p(0.0, 25.0, 0.0),
    ])?;
    let solid = extrude_profile_brep(&l, p(0.0, 0.0, 1.0), 12.0)?;
    // Each side by a point in its middle.
    let sides = faces_at(
        &solid,
        &[
            p(15.0, 0.0, 6.0), p(30.0, 5.0, 6.0), p(20.0, 10.0, 6.0),
            p(10.0, 17.5, 6.0), p(5.0, 25.0, 6.0), p(0.0, 12.5, 6.0),
        ],
    )?;
    Ok(draft_faces(&solid, &sides, p(0.0, 0.0, 1.0), p(0.0, 0.0, 0.0), 6f64.to_radians())?)
}
