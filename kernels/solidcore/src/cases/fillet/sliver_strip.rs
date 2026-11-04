//! A plate from the origin to (20, 10, 2), the two edges of its face y = 10 that run along x, at
//! z = 0 and z = 2, each rounded to radius 0.9999, leaving a strip of that face 0.0002 tall
//! between the rounds.
use crate::bench::{edges_through_each, reloaded};
use crate::Refusal;
use solidcore_ops::{blend, make_box, Blend, Body, Corners};

pub fn build() -> Result<Body, Refusal> {
    let (plate, order) = reloaded(&make_box([0.0, 0.0, 0.0], [20.0, 10.0, 2.0])?)?;
    let edges = edges_through_each(&plate, &order, &[[10.0, 10.0, 0.0], [10.0, 10.0, 2.0]])?;
    blend(&plate, Blend::Round, &edges, &[0.9999], &[], Corners::Filled)?;
    Ok(plate)
}
