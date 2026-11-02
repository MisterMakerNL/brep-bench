//! The box from the origin to (30, 20, 20) drilled along x by a hole of radius 4 on y = 10,
//! z = 10, its faces x = 0 and x = 30 then drafted 8° inward about z = 0.
use crate::sketch::polygon;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // cad-kernel drafts a profile's every side as it extrudes it, not two faces of a solid: the
    // box with its ends drafted is its section in the plane y = 0, a trapezoid, swept along y
    let inset = 20. * 8f64.to_radians().tan();
    let section = polygon(&[[0., 0., 0.], [inset, 0., 20.], [30. - inset, 0., 20.], [30., 0., 0.]]);
    let body = prism(&section, 0., 20.)?;
    // the hole, unmoved by the draft, cut from it; the drill runs past both ends
    let drill = prism(&disc([-1., 10., 10.], [1., 0., 0.], 4.), 0., 32.)?;
    Ok(combined(&body, &drill, Op::Cut, 0.)?)
}
