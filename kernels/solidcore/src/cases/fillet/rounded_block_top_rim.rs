//! A block from (−10, −6, 0) to (10, 6, 5) with its four vertical edges rounded to radius 3. The
//! rim of its top face, eight edges each running on tangent into the next (four straight, four
//! quarter circles), rounded to radius 1 all round.
use crate::bench::{edges_through_each, reloaded};
use crate::Refusal;
use solidcore_ops::{blend, make_box, Blend, Body, Corners};
use std::f64::consts::FRAC_PI_4;

pub fn build() -> Result<Body, Refusal> {
    let (block, order) = reloaded(&make_box([-10.0, -6.0, 0.0], [20.0, 12.0, 5.0])?)?;
    let verticals = edges_through_each(
        &block,
        &order,
        &[[10.0, 6.0, 2.5], [-10.0, 6.0, 2.5], [-10.0, -6.0, 2.5], [10.0, -6.0, 2.5]],
    )?;
    blend(&block, Blend::Round, &verticals, &[3.0], &[], Corners::Filled)?;

    // the top rim: its four straight edges and the four quarter circles between them
    let (block, order) = reloaded(&block)?;
    let (c, s) = (3.0 * FRAC_PI_4.cos(), 3.0 * FRAC_PI_4.sin());
    let rim = edges_through_each(
        &block,
        &order,
        &[
            [0.0, -6.0, 5.0],
            [7.0 + c, -3.0 - s, 5.0],
            [10.0, 0.0, 5.0],
            [7.0 + c, 3.0 + s, 5.0],
            [0.0, 6.0, 5.0],
            [-7.0 - c, 3.0 + s, 5.0],
            [-10.0, 0.0, 5.0],
            [-7.0 - c, -3.0 - s, 5.0],
        ],
    )?;
    blend(&block, Blend::Round, &rim, &[1.0], &[], Corners::Filled)?;
    Ok(block)
}
