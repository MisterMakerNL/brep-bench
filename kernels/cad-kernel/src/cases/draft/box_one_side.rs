//! The box from the origin to (20, 12, 10), its side x = 20 drafted 10° inward about z = 0.
use crate::sketch::polygon;
use cad_kernel::build::prism;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // cad-kernel drafts a profile's every side as it extrudes it, not one face of a solid: the box
    // with one side drafted is its section in the plane y = 0, a trapezoid, swept along y
    let top = 20. - 10. * 10f64.to_radians().tan();
    let section = polygon(&[[0., 0., 0.], [0., 0., 10.], [top, 0., 10.], [20., 0., 0.]]);
    prism(&section, 0., 12.)
}
