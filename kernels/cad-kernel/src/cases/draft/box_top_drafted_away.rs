//! The box from the origin to (30, 10, 10), its four sides drafted 30° inward about z = 0, which
//! consumes its top: a hip roof, its ridge at 5√3.
use crate::sketch::polygon;
use cad_kernel::build::drafted;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // the box's floor swept up to its top with its sides tapered from z = 0; past 5√3 the tapered
    // section is nothing
    let floor = polygon(&[[0., 0., 0.], [30., 0., 0.], [30., 10., 0.], [0., 10., 0.]]);
    drafted(&floor, 0., 10., 30f64.to_radians())
}
