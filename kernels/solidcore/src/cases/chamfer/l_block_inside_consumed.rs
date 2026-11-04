//! An L-block: the polygon through (0, 0), (10, 0), (10, 3), (3, 3), (3, 10) and (0, 10) in xz,
//! extruded along +y from y = 0 to y = 10. Its concave edge, the line x = 3, z = 3, chamfered
//! with distance 7 on both faces, adding material: the chamfer consumes both faces beside it.
use crate::bench::{edge_through, polygon, reloaded};
use crate::Refusal;
use solidcore_ops::sweep::{extrude, DraftCorner};
use solidcore_ops::{blend, Blend, Body, Corners};

pub fn build() -> Result<Body, Refusal> {
    let section = polygon(&[
        [0.0, 0.0, 0.0],
        [10.0, 0.0, 0.0],
        [10.0, 0.0, 3.0],
        [3.0, 0.0, 3.0],
        [3.0, 0.0, 10.0],
        [0.0, 0.0, 10.0],
    ]);
    let block = extrude(&section, [0.0, 1.0, 0.0], 10.0, 0.0, DraftCorner::default(), true)?;
    let (block, order) = reloaded(&block)?;
    let inside = edge_through(&block, &order, [3.0, 5.0, 3.0])?;
    blend(&block, Blend::Chamfer, &[inside], &[7.0], &[], Corners::Filled)?;
    Ok(block)
}
