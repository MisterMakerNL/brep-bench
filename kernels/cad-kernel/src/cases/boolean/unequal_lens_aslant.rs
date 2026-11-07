//! Balls of radius 5.511 about the origin and 3.317 about (4.231, 3.467, 2.653): the lens common to
//! both.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::sphere;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    // each ball's poles on the vertical through its centre
    let first = sphere([0., 0., 0.], [0., 0., 1.], 5.511)?;
    let second = sphere([4.231, 3.467, 2.653], [0., 0., 1.], 3.317)?;
    Ok(combined(&first, &second, Op::Common, 0.)?)
}
