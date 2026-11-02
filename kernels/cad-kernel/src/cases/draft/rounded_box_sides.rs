//! The box from the origin to (30, 20, 10), its vertical edges rounded to 4, its eight sides
//! drafted 5° inward about z = 0, the rounds becoming cones.
use crate::sketch::rounded_rectangle;
use cad_kernel::build::drafted;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // the rounded box's floor swept up with its sides tapered from z = 0: each line moves inward
    // and each arc's radius shrinks about its centre, so the cones stay tangent to the planes
    drafted(&rounded_rectangle([0., 0.], [30., 20.], 4., 0.), 0., 10., 5f64.to_radians())
}
