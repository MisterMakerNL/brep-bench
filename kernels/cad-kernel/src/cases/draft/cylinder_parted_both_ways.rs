//! The cylinder of radius 10 on z from z = 0 to 20, its side parted at z = 10 and each half
//! drafted 3° inward away from it.
use cad_kernel::build::{disc, drafted};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // the parting circle swept 10 each way with its side tapered away from it: cad-kernel's
    // drafted prism crossing its parting plane, two pieces glued at the crease
    drafted(&disc([0., 0., 10.], [0., 0., 1.], 10.), -10., 10., 3f64.to_radians())
}
