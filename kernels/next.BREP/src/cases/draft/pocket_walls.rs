//! The block from the origin to (40, 30, 15) with the pocket from (8, 8, 5) to (32, 22, 15) cut
//! into its top, the pocket's four walls drafted 5° for the pull direction +z about the neutral
//! plane z = 5, its floor: each wall turns about its bottom edge, leaning away from the pocket's
//! middle as it rises, so the pocket widens toward its opening.
use crate::solids::boolean::cut;
use crate::solids::draft::draft_side;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, 0.0, 0.0), 40.0, 30.0, 15.0)?;
    // The pocket's box runs past the block's top.
    let pocket = make_box_brep(p(8.0, 8.0, 5.0), 24.0, 14.0, 15.0)?;
    let mut solid = cut(&block, &pocket)?;
    // Each wall by a point in its middle, and the block's outward normal there, into the pocket.
    let walls = [
        (p(20.0, 8.0, 10.0), p(0.0, 1.0, 0.0)),
        (p(32.0, 15.0, 10.0), p(-1.0, 0.0, 0.0)),
        (p(20.0, 22.0, 10.0), p(0.0, -1.0, 0.0)),
        (p(8.0, 15.0, 10.0), p(1.0, 0.0, 0.0)),
    ];
    for (on_wall, normal) in walls {
        solid = draft_side(&solid, on_wall, normal, 5.0, 5f64.to_radians())?;
    }
    Ok(solid)
}
