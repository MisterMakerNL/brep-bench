//! An L-shaped plate, the polygon (0, 0), (20, 0), (20, 5), (5, 5), (5, 15), (0, 15) extruded
//! 8 up, shelled inward to a wall 1 thick, open at its top, the wall every point within 1 of its
//! faces other than the top: at the concave edge the inside is a quarter cylinder of radius 1
//! about the edge.
//!
//! geop's shell meets the moved faces sharp everywhere: it has no shell that rounds the wall at
//! a concave edge. So the plate is shelled sharp, and the inside edge left at x = 4, y = 4 is
//! then rounded to radius 1: the round tangent to the inside walls x = 4 and y = 4 is the
//! cylinder of radius 1 about the line x = 5, y = 5, which is the concave edge itself.
use crate::shared::hollow::{prism, shell};
use crate::{Bench, Refusal};
use geop_ops_fillet::FilletArgs;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let outline = [[0.0, 0.0], [20.0, 0.0], [20.0, 5.0], [5.0, 5.0], [5.0, 15.0], [0.0, 15.0]];
    prism(b, "plate", &outline, 8.0)?;
    shell(b, "cup", &[[2.0, 2.0, 8.0]], 1.0, false)?;
    let corner = b.edge([4.0, 4.0, 4.0])?;
    b.step("round", FilletArgs::constant(vec![corner], 1.0))
}
