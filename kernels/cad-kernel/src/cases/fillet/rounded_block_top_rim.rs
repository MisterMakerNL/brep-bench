//! A block from (−10, −6, 0) to (10, 6, 5), its vertical edges rounded to 3, the rim of its top
//! face rounded to 1: eight edges, straight and quarter circles, each running on tangent into the
//! next.
use crate::sketch::rounded_rectangle;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::prism;
use cad_kernel::fillet::roll;
use cad_kernel::geom::{tolerance, V};
use cad_kernel::topo::Brep;
use std::f64::consts::FRAC_1_SQRT_2;

const HALF: [f64; 2] = [10., 6.];
const CORNER: f64 = 3.;
const HEIGHT: f64 = 5.;
const ROUND: f64 = 1.;

pub fn build() -> Result<Brep, String> {
    let mut block = prism(&rounded_rectangle([-HALF[0], -HALF[1]], HALF, CORNER, 0.), 0., HEIGHT)?;
    // the rim's edges, each picked by its middle, in order round the top: fillet::roll rolls the
    // ball along the run between two faces, which here is one edge, so each is rolled in turn
    let [a, b] = HALF;
    let off = CORNER * (1. - FRAC_1_SQRT_2);
    let rim: [V; 8] = [
        [0., b, HEIGHT], [a - off, b - off, HEIGHT], [a, 0., HEIGHT], [a - off, -b + off, HEIGHT],
        [0., -b, HEIGHT], [-a + off, -b + off, HEIGHT], [-a, 0., HEIGHT], [-a + off, b - off, HEIGHT],
    ];
    for at in rim {
        let round = roll(&block, at, ROUND, tolerance(block.size()))?;
        let op = if round.concave { Op::Union } else { Op::Cut };
        block = combined(&block, &round.piece, op, 0.)?;
    }
    Ok(block)
}
