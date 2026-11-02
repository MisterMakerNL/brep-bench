//! The block from the origin to (40, 30, 15) less the pocket from (8, 8, 5) to (32, 22, 15), its
//! walls drafted 5° about z = 5 so that it widens toward its opening.
use crate::sketch::{block, rectangle};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::drafted;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let solid = block([0., 0., 0.], [40., 30., 15.])?;
    // the pocket's floor swept up past the top, its sides tapered out of it (a negative draft)
    let pocket = drafted(&rectangle([8., 8.], [32., 22.], 5.), 0., 11., -5f64.to_radians())?;
    Ok(combined(&solid, &pocket, Op::Cut, 0.)?)
}
