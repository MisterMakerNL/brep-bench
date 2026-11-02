//! The L through (0, 0), (30, 0), (30, 10), (10, 10), (10, 25) and (0, 25), 12 high, its six
//! sides drafted 6° inward about z = 0.
use crate::sketch::polygon;
use cad_kernel::build::drafted;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // the L swept up with its sides tapered from z = 0: each side moves into the material, and
    // each corner, the concave one too, is where the moved sides beside it cross
    let l = polygon(&[[0., 0., 0.], [30., 0., 0.], [30., 10., 0.], [10., 10., 0.], [10., 25., 0.], [0., 25., 0.]]);
    drafted(&l, 0., 12., 6f64.to_radians())
}
