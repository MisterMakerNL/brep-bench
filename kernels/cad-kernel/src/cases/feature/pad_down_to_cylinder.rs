//! A cylinder of radius 10 along the x axis from x = −20 to 20, with a square pad on top: the
//! square |x| ≤ 3, |y| ≤ 3 in the plane z = 15, extruded down until it meets the cylinder's side,
//! and joined to it.
//!
//! cad-kernel's prism runs between two given heights and has no extrusion up to a face. So the
//! square is extruded down 15, to the cylinder's axis: everything of it below the cylinder's side
//! lies inside the cylinder, and the union leaves the pad ending on the side.
use crate::sketch::rectangle;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

const RADIUS: f64 = 10.;
const HALF_LENGTH: f64 = 20.;
const HALF_SIDE: f64 = 3.;
const TOP: f64 = 15.;

pub fn build() -> Result<Brep, String> {
    let cylinder = prism(&disc([-HALF_LENGTH, 0., 0.], [1., 0., 0.], RADIUS), 0., 2. * HALF_LENGTH)?;
    // the square faces +z; swept from its plane down to the axis
    let square = rectangle([-HALF_SIDE, -HALF_SIDE], [HALF_SIDE, HALF_SIDE], TOP);
    let pad = prism(&square, -TOP, 0.)?;
    Ok(combined(&cylinder, &pad, Op::Union, 0.)?)
}
