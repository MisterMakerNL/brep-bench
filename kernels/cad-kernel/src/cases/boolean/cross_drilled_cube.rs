//! A cube from the origin to (30, 30, 30), drilled right through by fifteen holes of radius 2.5, one
//! after another: three along x at y = 7.5, 15, 22.5 and z = 15, three along y at x = 7.5, 15, 22.5
//! and z = 15, and nine along z through (x, y) for x and y each 7.5, 15, 22.5.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

const SIDE: f64 = 30.;
const RADIUS: f64 = 2.5;
const GRID: [f64; 3] = [7.5, 15., 22.5];

pub fn build() -> Result<Brep, String> {
    let mut cube = block([0., 0., 0.], [SIDE, SIDE, SIDE])?;
    // each drill a disc at one end, square to its axis, swept a unit past the cube at both ends
    let mut drills = Vec::new();
    for g in GRID {
        drills.push(([-1., g, 15.], [1., 0., 0.]));
        drills.push(([g, -1., 15.], [0., 1., 0.]));
    }
    for x in GRID {
        for y in GRID {
            drills.push(([x, y, -1.], [0., 0., 1.]));
        }
    }
    for (start, along) in drills {
        let drill = prism(&disc(start, along, RADIUS), 0., SIDE + 2.)?;
        cube = combined(&cube, &drill, Op::Cut, 0.)?;
    }
    Ok(cube)
}
