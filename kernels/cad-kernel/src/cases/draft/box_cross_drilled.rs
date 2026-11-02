//! The box from the origin to (30, 20, 20) drilled along x by a hole of radius 4 on y = 10,
//! z = 10, its faces x = 0 and x = 30 then drafted 8° inward about z = 0.
use crate::sketch::{block, drafted_face};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{disc, prism};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // the drill runs past both ends
    let drill = prism(&disc([-1., 10., 10.], [1., 0., 0.], 4.), 0., 32.)?;
    let drilled = combined(&block([0., 0., 0.], [30., 20., 20.])?, &drill, Op::Cut, 0.)?;
    // cad-kernel drafts a profile as it extrudes it, not a solid's faces: each end is drafted by
    // keeping the drilled box inside its plane turned about its bottom edge, which trims the hole
    let draft = 8f64.to_radians();
    let one_end = drafted_face(&drilled, [0., 0., 0.], [-1., 0., 0.], draft)?;
    drafted_face(&one_end, [30., 0., 0.], [1., 0., 0.], draft)
}
