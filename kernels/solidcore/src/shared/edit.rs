//! What the solidcore implementations of the edit cases share.
use crate::bench::{cut, faces_through_each, order};
use crate::Refusal;
use solidcore_ops::{make_box, make_cylinder, offset_faces, Body};

/// The box from the origin to `size`, drilled right through along z by a hole of `radius` on the
/// vertical line through (`x`, `y`): a cylinder running past both faces, cut from the box.
pub fn drilled_box(size: [f64; 3], x: f64, y: f64, radius: f64) -> Result<Body, Refusal> {
    let block = make_box([0.0, 0.0, 0.0], size)?;
    let drill = make_cylinder([x, y, -1.0], radius, size[2] + 2.0)?;
    cut(block, drill)
}

/// The faces of `body` through `points` moved `distance` along their outward normals (negative:
/// inward), the faces around them following: ops' face offset, solidcore's direct edit.
pub fn move_faces(body: &Body, points: &[[f64; 3]], distance: f64) -> Result<(), Refusal> {
    let faces = faces_through_each(body, points)?;
    offset_faces(body, &order(body)?, &faces, distance)?;
    Ok(())
}
