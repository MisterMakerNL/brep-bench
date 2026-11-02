//! A cube of side 10 between the origin and (10, 10, 10), the edge where its faces x = 10 and
//! y = 10 meet, from (10, 10, 0) to (10, 10, 10), rounded with a radius growing linearly along
//! it: r = 1 + z/10, 1 at the bottom and 2 at the top.
use crate::bench::{edge_through, p};
use crate::Refusal;
use kentron::blend;
use kentron::geometry::ParametricCurve;
use kentron::primitives::Cuboid;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let cube = Cuboid::from_corners(p(0.0, 0.0, 0.0), p(10.0, 10.0, 10.0)).build(m)?;
    let edge = edge_through(m, cube, p(10.0, 10.0, 5.0))?;
    // The law runs from the edge's start to its end: each end gets the radius at its height.
    let (curve, first, _) = m.edge_curve(edge).ok_or("the edge has no curve")?;
    let law = if curve.point(first).z < 5.0 { vec![(0.0, 1.0), (1.0, 2.0)] } else { vec![(0.0, 2.0), (1.0, 1.0)] };
    Ok(blend::fillet_evolving(m, cube, &[(edge, law)])?)
}
