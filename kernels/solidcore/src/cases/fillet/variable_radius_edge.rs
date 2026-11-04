//! A cube of side 10 from the origin to (10, 10, 10), the edge where its faces x = 10 and y = 10
//! meet rounded with a radius growing linearly along it, r = 1 + z/10: 1 at z = 0, 2 at z = 10.
use crate::bench::{edge_through, reloaded};
use crate::shared::fillet::vertex_at;
use crate::Refusal;
use solidcore_ops::{fillet_variable, make_box, Body};

pub fn build() -> Result<Body, Refusal> {
    let (cube, order) = reloaded(&make_box([0.0, 0.0, 0.0], [10.0, 10.0, 10.0])?)?;
    let edge = edge_through(&cube, &order, [10.0, 10.0, 5.0])?;
    // the law runs from the edge's end at the bottom face: linear, from 1 to 2
    let bottom = vertex_at(&cube, &order, [10.0, 10.0, 0.0])?;
    fillet_variable(&cube, edge, bottom, 1, &[0.0, 0.0, 1.0, 1.0], &[1.0, 2.0])?;
    Ok(cube)
}
