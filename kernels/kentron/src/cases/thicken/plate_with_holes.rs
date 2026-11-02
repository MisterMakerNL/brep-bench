//! A sheet, the square in the xy plane from (−10, −10, 0) to (10, 10, 0) with two round holes of
//! radius 3 centred on (−5, 0, 0) and (5, 0, 0), thickened by 1 to each side: the plate from
//! z = −1 to z = 1 holed right through by two cylinders of radius 3.
//!
//! The sheet is thickened to one side only, so it is first moved 1 to its −z side (an offset of
//! the face alone) and that thickened by 2 to +z, as OCCT's implementation does.
use crate::bench::{frame, p};
use crate::shared::offset::polygon_face;
use crate::shared::thicken::thickened;
use crate::Refusal;
use kentron::geometry::curve::{Circle, Curve};
use kentron::make;
use kentron::math::Direction;
use kentron::offset::{self, Join};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    // anticlockwise seen from +z: its normal is +z
    let square = polygon_face(m, &[p(-10.0, -10.0, 0.0), p(10.0, -10.0, 0.0), p(10.0, 10.0, 0.0), p(-10.0, 10.0, 0.0)])?;
    let mut holes = Vec::new();
    for x in [-5.0, 5.0] {
        // a hole runs clockwise seen from +z: about −z
        let circle = Circle::new(frame(p(x, 0.0, 0.0), -Direction::Z), 3.0);
        let edge = make::edge_from_curve(m, Curve::Circle(circle))?;
        holes.push(make::wire(m, &[edge])?);
    }
    let sheet = make::face_with_holes(m, square, &holes)?;
    let lower = offset::shape(m, sheet, -1.0, Join::Arc)?;
    thickened(m, lower, 2.0, Join::Arc)
}
