//! A cube of side 10 between the origin and (10, 10, 10), the edge where its faces x = 10 and
//! y = 10 meet rounded to radius 2.
use crate::bench::{edge_through, reloaded};
use crate::Refusal;
use solidcore_ops::{blend, make_box, Blend, Body, Corners};

pub fn build() -> Result<Body, Refusal> {
    let (cube, order) = reloaded(&make_box([0.0, 0.0, 0.0], [10.0, 10.0, 10.0])?)?;
    let edge = edge_through(&cube, &order, [10.0, 10.0, 5.0])?;
    blend(&cube, Blend::Round, &[edge], &[2.0], &[], Corners::Filled)?;
    Ok(cube)
}
