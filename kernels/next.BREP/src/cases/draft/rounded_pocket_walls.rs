//! The block from the origin to (40, 30, 15) with a pocket cut into its top from (8, 8, 5) to
//! (32, 22, 15), its vertical edges rounded to radius 4, all eight walls drafted 5° for the pull
//! direction +z about the neutral plane z = 5, its floor: the kernel's draft turns each plane wall
//! about its bottom edge and makes each round the quarter cone through its quarter circle, the
//! pocket widening as it rises.
use crate::solids::boolean::cut;
use crate::solids::draft::{rounded_rectangle, walls};
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{draft_faces, extrude_profile_brep, make_box_brep, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, 0.0, 0.0), 40.0, 30.0, 15.0)?;
    // The pocket's cutter runs past the block's top.
    let floor = rounded_rectangle(8.0, 8.0, 32.0, 22.0, 4.0, 5.0)?;
    let pocket = extrude_profile_brep(&floor, p(0.0, 0.0, 1.0), 15.0)?;
    let solid = cut(&block, &pocket)?;
    // The pocket's walls: the faces along z inside the pocket's outline.
    let pocket_walls = walls(&solid, |c| c.x > 7.0 && c.x < 33.0 && c.y > 7.0 && c.y < 23.0)?;
    Ok(draft_faces(&solid, &pocket_walls, p(0.0, 0.0, 1.0), p(0.0, 0.0, 5.0), 5f64.to_radians())?)
}
