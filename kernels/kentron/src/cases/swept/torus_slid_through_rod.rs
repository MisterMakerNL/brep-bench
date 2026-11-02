//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, grooved by a wheel slid
//! past it: the torus with its axis parallel to the y axis, radii 6 and 2, slid without turning
//! from centre (11.5, 0, −9), wholly below the rod, to (11.5, 0, 29), wholly above it. Everything
//! the wheel passes through, cut.
//!
//! kentron has no sweep of a solid (`sweep::extrude` takes faces, wires, edges, vertices and
//! compounds of them), so the wheel's sweep is built as elementary geometry gives it: the wheel at
//! each end, and the quarters of its face, split along the circles where the slide runs along it,
//! each slid (`shared::swept::slid_torus`). They are cut from the rod in turn.
use crate::bench::{p, v};
use crate::shared::swept::{cut_each, slid_torus};
use kentron::primitives::Cylinder;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let rod = Cylinder::new(5.0, 20.0).build(m)?;
    let pieces = slid_torus(m, p(11.5, 0.0, -9.0), 6.0, 2.0, v(0.0, 0.0, 38.0))?;
    cut_each(m, rod, &pieces)
}
