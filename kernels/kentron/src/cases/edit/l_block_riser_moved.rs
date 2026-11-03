//! The L-block whose section is the polygon through (x, z) = (0, 0), (40, 0), (40, 10), (15, 10),
//! (15, 30), (0, 30) in the plane y = 0, extruded along +y to y = 20. Its riser, the face x = 15
//! from z = 10 to z = 30, is moved out by 10 along +x, to x = 25; the foot's top, the upright's top
//! and the two sides are cut back or run on to meet it.
use crate::bench::{face_through, p, v};
use crate::shared::edit::moved;
use crate::shared::offset::polygon_face;
use crate::Refusal;
use kentron::sweep;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let section = [(0.0, 0.0), (40.0, 0.0), (40.0, 10.0), (15.0, 10.0), (15.0, 30.0), (0.0, 30.0)];
    let corners: Vec<_> = section.iter().map(|&(x, z)| p(x, 0.0, z)).collect();
    let face = polygon_face(m, &corners)?;
    let block = sweep::extrude(m, face, v(0.0, 20.0, 0.0))?.shape;
    let riser = face_through(m, block, p(15.0, 10.0, 20.0))?;
    Ok(moved(m, block, &[(riser, 10.0)])?) // its outward normal is +x
}
