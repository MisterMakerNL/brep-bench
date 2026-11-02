//! A solid ellipsoid about the origin of semi-axes 6, 4 and 3 along x, y and z.
use cad_kernel::build::ellipsoid;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // the kernel's ellipsoid: its third axis along z, its first along x
    ellipsoid([0., 0., 0.], [0., 0., 1.], [1., 0., 0.], [6., 4., 3.])
}
