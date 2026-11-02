//! A solid torus about the z axis centred on the origin, middle circle of radius 5 and tube of
//! radius 3, cut by the plane 3x − 4z = 0 and kept on and below it (z ≤ 3x/4). The plane is
//! bitangent to the torus, touching it at (±16/5, 0, ±12/5), and cuts it in Villarceau's circles.
//! The torus's seam (where its tube's section was turned from) is in the plane x = 0 on the side
//! y > 0, away from the tangent points. What is cut away is the half-space above the plane,
//! bounded by a square of the plane reaching well past the torus.
use crate::bench::{d, p};
use kentron::boolean;
use kentron::geometry::surface::{Plane, Surface};
use kentron::make;
use kentron::math::{Direction, Frame};
use kentron::primitives::{half_space, Torus};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let torus = Torus::new(5.0, 3.0).with_frame(Frame::new(p(0.0, 0.0, 0.0), Direction::Z, Direction::Y)).build(m)?;
    let plane = Plane::new(p(0.0, 0.0, 0.0), d(-3.0, 0.0, 4.0));
    let face = make::face_on_domain(m, Surface::Plane(plane), -50.0, 50.0, -50.0, 50.0)?;
    // the side of the plane its normal (−3, 0, 4) points to
    let above = half_space(m, face, p(-3.0, 0.0, 4.0))?;
    boolean::difference(m, torus, above)
}
