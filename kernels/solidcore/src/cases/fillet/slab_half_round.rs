//! A block from the origin to (10, 4, 6), the two edges of its top face that run along x, at
//! y = 0 and y = 4, each rounded to radius 2, half the top's width: the top vanishes and the
//! block's top is a half-cylinder.
use crate::bench::{edges_through_each, reloaded};
use crate::Refusal;
use solidcore_ops::{blend, make_box, Blend, Body, Corners};

pub fn build() -> Result<Body, Refusal> {
    let (block, order) = reloaded(&make_box([0.0, 0.0, 0.0], [10.0, 4.0, 6.0])?)?;
    let edges = edges_through_each(&block, &order, &[[5.0, 0.0, 6.0], [5.0, 4.0, 6.0]])?;
    blend(&block, Blend::Round, &edges, &[2.0], &[], Corners::Filled)?;
    Ok(block)
}
