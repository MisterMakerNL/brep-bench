//! An L-block: the polygon through (x, z) = (0, 0), (40, 0), (40, 10), (15, 10), (15, 30),
//! (0, 30) in the plane y = 0, extruded along +y to y = 20; then its riser, the face x = 15 from
//! z = 10 to z = 30, moved out by 10 along +x to x = 25, the faces beside it following.
use crate::bench::polygon;
use crate::shared::edit::move_faces;
use crate::Refusal;
use solidcore_ops::sweep::{extrude, DraftCorner};
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    let section = polygon(&[
        [0.0, 0.0, 0.0],
        [40.0, 0.0, 0.0],
        [40.0, 0.0, 10.0],
        [15.0, 0.0, 10.0],
        [15.0, 0.0, 30.0],
        [0.0, 0.0, 30.0],
    ]);
    let block = extrude(&section, [0.0, 1.0, 0.0], 20.0, 0.0, DraftCorner::default(), true)?;
    // the riser faces +x, out of the material
    move_faces(&block, &[[15.0, 10.0, 20.0]], 10.0)?;
    Ok(block)
}
