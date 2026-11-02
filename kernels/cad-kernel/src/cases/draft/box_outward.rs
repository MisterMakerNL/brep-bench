//! The box from the origin to (20, 12, 10), its four sides drafted 5° outward about z = 0.
use crate::sketch::polygon;
use cad_kernel::build::drafted;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // the box's floor swept up with its sides tapered from z = 0; a negative draft moves the
    // section out of its material as it rises
    let floor = polygon(&[[0., 0., 0.], [20., 0., 0.], [20., 12., 0.], [0., 12., 0.]]);
    drafted(&floor, 0., 10., -5f64.to_radians())
}
