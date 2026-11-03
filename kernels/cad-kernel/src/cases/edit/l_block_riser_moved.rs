//! The L-block whose section is the polygon through (x, z) = (0, 0), (40, 0), (40, 10), (15, 10),
//! (15, 30), (0, 30) in the plane y = 0, extruded along +y to y = 20, its riser (the face x = 15
//! from z = 10 to 30) then moved out by 10 along +x, to x = 25.
use crate::sketch::{faces_through, polygon};
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::prism;
use cad_kernel::offset::normal_sweep;
use cad_kernel::topo::Brep;

pub fn build() -> Result<Brep, String> {
    let section = polygon(&[[0., 0., 0.], [40., 0., 0.], [40., 0., 10.], [15., 0., 10.],
                            [15., 0., 30.], [0., 0., 30.]]);
    // walked this way round, the polygon's normal is −y: y = 0 to 20 is −20 to 0 along it
    let l = prism(&section, -20., 0.)?;
    // cad-kernel moves no face of a solid. Every face beside the riser (the foot's top, the
    // upright's top, the two sides) is square to it, so carried on along the move they bound
    // exactly what the riser sweeps through as it moves: the moved solid is the block with the
    // riser's sweep along its normal added.
    let riser = faces_through(&l, &[[15., 10., 20.]])?[0];
    Ok(combined(&l, &normal_sweep(&l, riser, [0., 10.])?, Op::Union, 0.)?)
}
