//! The L-block whose section is the polygon through (x, z) = (0, 0), (40, 0), (40, 10), (15, 10),
//! (15, 30), (0, 30) in the plane y = 0, extruded along +y to y = 20. Its riser, the face x = 15
//! from z = 10 to z = 30, is moved out by 10 along +x, to x = 25; the foot's top, the upright's top
//! and the two sides are cut back or run on to meet it.
use crate::solids::shell::face_at;
use crate::solids::{p, polygon};
use crate::Refusal;
use brep_kernel::{extrude_profile_brep, move_faces, BrepSolid};

pub fn build() -> Result<BrepSolid, Refusal> {
    let section = polygon(&[
        p(0.0, 0.0, 0.0), p(40.0, 0.0, 0.0), p(40.0, 0.0, 10.0),
        p(15.0, 0.0, 10.0), p(15.0, 0.0, 30.0), p(0.0, 0.0, 30.0),
    ])?;
    let block = extrude_profile_brep(&section, p(0.0, 1.0, 0.0), 20.0)?;
    let riser = face_at(&block, p(15.0, 10.0, 20.0))?;
    Ok(move_faces(&block, &[riser], p(10.0, 0.0, 0.0))?)
}
