//! The solid the box (0, 0, 0)–(10, 6, 4) fills as it slides without turning along the vector
//! (8, 5, 3), to the box (8, 5, 3)–(18, 11, 7).
use crate::sketch::block;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::swept::closed::slid;
use cad_kernel::topo::Brep;
use cad_kernel::trajectory::{Screw, Trajectory};
use cad_kernel::util::space::{norm, unit};
use std::f64::consts::TAU;

const SIZE: V = [10., 6., 4.];
const SLIDE: V = [8., 5., 3.];

pub fn build() -> Result<Brep, String> {
    let tool = block([0., 0., 0.], SIZE)?;
    // the slide along (8, 5, 3) over the roll [0, 1]: a screw that does not turn, advancing its
    // whole length a roll of τ
    let path = Trajectory::Twist(Screw {
        origin: [0., 0., 0.],
        axis: unit(SLIDE),
        ratio: 0.,
        advance: norm(SLIDE) * TAU,
    });
    // the kernel's closed swept solid; membership read on the box meshed to a thousandth of it
    let size = tool.size();
    slid(&tool, &path, [0., 1.], 1e-3 * size, tolerance(size))
        .map_err(|why| format!("the box's sweep not built: {why:?}"))
}
