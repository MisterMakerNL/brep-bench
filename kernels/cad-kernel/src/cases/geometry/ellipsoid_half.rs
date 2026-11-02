//! The ellipsoid x²/36 + y²/16 + z²/9 ≤ 1 with everything below z = 0 cut away.
use cad_kernel::build::{ellipsoid, kept};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let whole = ellipsoid([0., 0., 0.], [0., 0., 1.], [1., 0., 0.], [6., 4., 3.])?;
    // what stands on the plane z = 0, through its widest section
    kept(&whole, [0., 0., 0.], [0., 0., 1.], 0.)
}
