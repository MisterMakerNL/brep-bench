//! The box from the origin to (20, 12, 10), its four sides drafted 5° for the pull direction +z
//! about the neutral plane z = 0, the floor: each side turns about its bottom edge, leaning inward
//! as it rises.
use crate::solids::draft::draft_side;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let mut solid = make_box_brep(p(0.0, 0.0, 0.0), 20.0, 12.0, 10.0)?;
    // Each side by a point in its middle, and its outward normal.
    let sides = [
        (p(10.0, 0.0, 5.0), p(0.0, -1.0, 0.0)),
        (p(20.0, 6.0, 5.0), p(1.0, 0.0, 0.0)),
        (p(10.0, 12.0, 5.0), p(0.0, 1.0, 0.0)),
        (p(0.0, 6.0, 5.0), p(-1.0, 0.0, 0.0)),
    ];
    for (on_side, normal) in sides {
        solid = draft_side(&solid, on_side, normal, 0.0, 5f64.to_radians())?;
    }
    Ok(solid)
}
