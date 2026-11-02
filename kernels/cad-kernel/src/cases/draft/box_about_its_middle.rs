//! The box from the origin to (20, 12, 10), its four sides drafted 5° inward about z = 5.
use crate::sketch::{drafted_through, rectangle};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // cad-kernel drafts a profile as it extrudes it, not a solid's faces: the box's section at
    // the neutral plane, z = 5, swept 5 down and 5 up with its sides leaning inward as they rise
    drafted_through(&rectangle([0., 0.], [20., 12.], 5.), 5., 5., 5f64.to_radians())
}
