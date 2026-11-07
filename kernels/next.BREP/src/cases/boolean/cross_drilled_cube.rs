//! A cube from the origin to (30, 30, 30), drilled right through by fifteen holes of radius 2.5,
//! one after another, each its own cut by a cylinder reaching 1 past the cube at both ends: three
//! along x at y = 7.5, 15, 22.5 and z = 15, three along y at x = 7.5, 15, 22.5 and z = 15, and nine
//! along z through (x, y) for x and y each 7.5, 15, 22.5.
use crate::solids::boolean::cut;
use crate::solids::p;
use crate::Refusal;
use brep_kernel::{make_box_brep, make_cylinder_brep, BrepSolid};

const SIDE: f64 = 30.0;
const RADIUS: f64 = 2.5;
const GRID: [f64; 3] = [7.5, 15.0, 22.5];

pub fn build() -> Result<BrepSolid, Refusal> {
    let mut cube = make_box_brep(p(0.0, 0.0, 0.0), SIDE, SIDE, SIDE)?;
    let mut drills = Vec::new();
    for g in GRID {
        drills.push((p(-1.0, g, 15.0), p(1.0, 0.0, 0.0)));
        drills.push((p(g, -1.0, 15.0), p(0.0, 1.0, 0.0)));
    }
    for x in GRID {
        for y in GRID {
            drills.push((p(x, y, -1.0), p(0.0, 0.0, 1.0)));
        }
    }
    for (start, along) in drills {
        cube = cut(&cube, &make_cylinder_brep(start, along, RADIUS, SIDE + 2.0)?)?;
    }
    Ok(cube)
}
