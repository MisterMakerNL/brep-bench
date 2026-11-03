//! The box from the origin to (40, 30, 10), the edge where its top, z = 10, meets its back,
//! y = 30, rounded to radius 3. Its top is moved up by 6, to z = 16, carrying the round with it:
//! the round stays a quarter cylinder of radius 3 tangent to the top and the back, its axis moved
//! from y = 27, z = 7 up to z = 13, and the back and the ends run on up to meet them.
use crate::bench::{edge_through, face_through, p};
use crate::shared::edit::moved;
use crate::Refusal;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};
use kentron::{blend, defeature};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let block = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(40.0, 30.0, 10.0)).build(m)?;
    let edge = edge_through(m, block, p(20.0, 30.0, 10.0))?;
    let rounded = blend::fillet(m, block, &[edge], 3.0)?;
    // kentron moves a face only by offsetting it, and offsetting the top alone would leave the
    // round where it is, to be stretched. So the round is removed, the top moved, and the edge
    // rounded again where the top now meets the back: the round carried up with the top.
    let k = 3.0 * (1.0 - 0.5f64.sqrt());
    let round = face_through(m, rounded, p(20.0, 30.0 - k, 10.0 - k))?;
    let sharp = defeature::remove_faces(m, rounded, &[round])?.shape;
    let top = face_through(m, sharp, p(20.0, 15.0, 10.0))?;
    let raised = moved(m, sharp, &[(top, 6.0)])?;
    let edge = edge_through(m, raised, p(20.0, 30.0, 16.0))?;
    Ok(blend::fillet(m, raised, &[edge], 3.0)?)
}
