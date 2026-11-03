//! The box from the origin to (40, 30, 10), the edge where its top, z = 10, meets its back,
//! y = 30, rounded to radius 3. Its top is moved up by 6, to z = 16, carrying the round with it:
//! the round stays a quarter cylinder of radius 3 tangent to the top and the back, its axis moved
//! from y = 27, z = 7 up to z = 13, and the back and the ends run on up to meet them.
use crate::solids::p;
use crate::solids::shell::face_at;
use crate::Refusal;
use brep_kernel::{fillet_edges, make_box_brep, move_faces, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let block = make_box_brep(p(0.0, 0.0, 0.0), 40.0, 30.0, 10.0)?;
    let rounded = fillet_edges(&block, &[p(20.0, 30.0, 10.0)], None, 3.0, false, None)?;
    // The top and the round move together, as one group: translated rigidly, the round keeps its
    // radius and stays tangent to the top, and the back and the ends, parallel to the move, slide
    // up to meet them.
    let k = 3.0 * (1.0 - 0.5f64.sqrt());
    let top = face_at(&rounded, p(20.0, 15.0, 10.0))?;
    let round = face_at(&rounded, p(20.0, 30.0 - k, 10.0 - k))?;
    Ok(move_faces(&rounded, &[top, round], p(0.0, 0.0, 6.0))?)
}
