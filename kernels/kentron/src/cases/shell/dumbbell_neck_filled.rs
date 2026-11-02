//! A dumbbell-shaped plate, two 10 × 10 squares joined by a neck 4 long and 2 wide (the outline
//! below), extruded 6 along +z, shelled inward to a wall 1.5 thick, open at its top face, z = 6:
//! the wall is every point of the plate within 1.5 of its other faces. The neck is narrower than
//! twice the wall, so it is left solid, and the cavity is two pockets, each bulging towards the
//! neck by arcs of radius 1.5 about the neck's corners.
use crate::bench::p;
use crate::shared::shell;
use crate::Refusal;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};

const OUTLINE: [(f64, f64); 12] = [
    (0.0, 0.0),
    (10.0, 0.0),
    (10.0, 4.0),
    (14.0, 4.0),
    (14.0, 0.0),
    (24.0, 0.0),
    (24.0, 10.0),
    (14.0, 10.0),
    (14.0, 6.0),
    (10.0, 6.0),
    (10.0, 10.0),
    (0.0, 10.0),
];

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let plate = shell::plate(m, &OUTLINE, 6.0)?;
    // joined by arcs: the wall is the points within 1.5 of the faces, rounded where they part
    shell::inward(m, plate, &[p(5.0, 5.0, 6.0)], 1.5, Join::Arc)
}
