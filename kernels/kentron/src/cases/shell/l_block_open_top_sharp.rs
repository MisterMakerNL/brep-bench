//! An L-shaped plate, the polygon through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15)
//! extruded 8 along +z, shelled inward to a wall 1 thick, open at its top face, z = 8, every pair of
//! moved faces extended until they meet, the concave edge included: the cavity is the polygon
//! through (1, 1), (19, 1), (19, 4), (4, 4), (4, 14) and (1, 14), from z = 1 up to the top.
use crate::bench::p;
use crate::shared::shell;
use crate::Refusal;
use kentron::offset::Join;
use kentron::topology::{Model, Shape};

const OUTLINE: [(f64, f64); 6] = [(0.0, 0.0), (20.0, 0.0), (20.0, 5.0), (5.0, 5.0), (5.0, 15.0), (0.0, 15.0)];

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let plate = shell::plate(m, &OUTLINE, 8.0)?;
    // joined by intersection: the moved walls at the concave edge extended until they meet
    shell::inward(m, plate, &[p(2.0, 2.0, 8.0)], 1.0, Join::Intersection)
}
