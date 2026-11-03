//! A pipe of radius 3 along two straight runs, (−30, 0, 0) to the origin and on along +y to
//! (0, 30, 0), the disc carried round the sharp corner by turning it about the vertical line
//! through the corner, which fills the wedge outside the corner with a quarter ball.
//!
//! cad-kernel has no sweep along a path with corners, so the pipe is built as the description
//! composes it: each run is the disc swept straight along it, ending flat at the corner, and the
//! joint is the disc turned a quarter about the z axis, a quarter of the ball of radius 3 about the
//! origin. The rest of that ball lies inside the two runs, so the pipe is the runs and the ball
//! joined. (The ball's half disc turned about z alone is refused, its arc's ends on the axis; and a
//! ball whose poles lie on z puts them where the runs' end rims cross, which the union leaves
//! open. Its axis is the bisector of the wedge outside the corner instead.)
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism, sphere};
use cad_kernel::topo::Brep;

const RADIUS: f64 = 3.;
const RUN: f64 = 30.;

pub fn build() -> Result<Brep, String> {
    let first = prism(&disc([-RUN, 0., 0.], [1., 0., 0.], RADIUS), 0., RUN)?;
    let second = prism(&disc([0., 0., 0.], [0., 1., 0.], RADIUS), 0., RUN)?;
    let joint = sphere([0., 0., 0.], [1., -1., 0.], RADIUS)?;
    let runs = combined(&first, &second, Op::Union, 0.)?;
    Ok(combined(&runs, &joint, Op::Union, 0.)?)
}
