//! The box from the origin to (20, 12, 10), its four sides drafted 5° inward about z = 0.
use crate::sketch::polygon;
use cad_kernel::build::drafted;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // cad-kernel drafts a profile as it extrudes it, not a solid's faces: the box's floor, swept up
    // with its sides tapered from the plane z = 0, is the box with its sides drafted
    let floor = polygon(&[[0., 0., 0.], [20., 0., 0.], [20., 12., 0.], [0., 12., 0.]]);
    drafted(&floor, 0., 10., 5f64.to_radians())
}
