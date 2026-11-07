//! A cylinder of radius 5 on z from z = −1 to 10, kept above z = 0 and below z = x tan 30°, the
//! plane through the y axis rising toward +x.
use cad_kernel::build::{disc, kept, prism};
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let cylinder = prism(&disc([0., 0., -1.], [0., 0., 1.], 5.), 0., 11.)?;
    let above = kept(&cylinder, [0., 0., 0.], [0., 0., 1.], 0.)?;
    // z ≤ x tan 30°: the side of the plane its normal (sin 30°, 0, −cos 30°) points to
    let (s, c) = 30f64.to_radians().sin_cos();
    kept(&above, [0., 0., 0.], [s, 0., -c], 0.)
}
