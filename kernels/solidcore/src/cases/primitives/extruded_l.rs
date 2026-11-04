//! An L-shaped plate: the polygon through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15) in
//! the xy plane, extruded 8 along +z.
use crate::bench::polygon;
use solidcore_ops::sweep::{extrude, DraftCorner};
use solidcore_ops::{Body, Result};

pub fn build() -> Result<Body> {
    let outline = polygon(&[
        [0.0, 0.0, 0.0],
        [20.0, 0.0, 0.0],
        [20.0, 5.0, 0.0],
        [5.0, 5.0, 0.0],
        [5.0, 15.0, 0.0],
        [0.0, 15.0, 0.0],
    ]);
    extrude(&outline, [0.0, 0.0, 1.0], 8.0, 0.0, DraftCorner::default(), true)
}
