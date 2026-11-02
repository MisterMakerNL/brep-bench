//! The regular tetrahedron with corners (3, 3, 3), (3, −3, −3), (−3, 3, −3) and (−3, −3, 3),
//! grown by 1: every point within 1 of it, its edges pieces of cylinders and its corners pieces
//! of balls. The tetrahedron is the solid its four triangles enclose.
use crate::bench::p;
use crate::shared::offset::polygon_face;
use kentron::boolean;
use kentron::offset::{self, Join};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let corners = [p(3.0, 3.0, 3.0), p(3.0, -3.0, -3.0), p(-3.0, 3.0, -3.0), p(-3.0, -3.0, 3.0)];
    let mut faces = Vec::new();
    for left_out in 0..4 {
        let triangle: Vec<_> = (0..4).filter(|&i| i != left_out).map(|i| corners[i]).collect();
        faces.push(polygon_face(m, &triangle)?);
    }
    let tetrahedron = boolean::enclosed_solids(m, &faces)?;
    offset::shape(m, tetrahedron, 1.0, Join::Arc)
}
