//! A cylinder of radius 5 on the z axis, from z = 0 to z = 10, the circular edge around its top
//! face rounded to radius 1.
use crate::bench::{edge_through, reloaded};
use crate::Refusal;
use solidcore_ops::{blend, make_cylinder, Blend, Body, Corners};

pub fn build() -> Result<Body, Refusal> {
    let (cylinder, order) = reloaded(&make_cylinder([0.0, 0.0, 0.0], 5.0, 10.0)?)?;
    let rim = edge_through(&cylinder, &order, [0.0, 5.0, 10.0])?;
    blend(&cylinder, Blend::Round, &[rim], &[1.0], &[], Corners::Filled)?;
    Ok(cylinder)
}
