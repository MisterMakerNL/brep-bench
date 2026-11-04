//! The octahedron |x| + |y| + |z| ≤ 10, all twelve edges chamfered with distance √6 on both
//! faces, the corners mitred. The octahedron is built from datum planes through its corners and
//! the slabs between its opposite faces, intersected (see [`octahedron`]); then one chamfer of
//! its twelve edges, each picked by its midpoint: geop bevels them one after the other, each tool
//! built from the octahedron, and leaves each corner to its four chamfers, which cross in a point.
use crate::shared::blends::octahedron;
use crate::{Bench, Refusal};
use geop_ops_fillet::ChamferArgs;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    octahedron(b, "octa", 10.0)?;
    // Each edge's midpoint: four round the middle, four above it and four below.
    let edges = b.edges(&[
        [5.0, 5.0, 0.0], [-5.0, 5.0, 0.0], [-5.0, -5.0, 0.0], [5.0, -5.0, 0.0],
        [5.0, 0.0, 5.0], [-5.0, 0.0, 5.0], [0.0, 5.0, 5.0], [0.0, -5.0, 5.0],
        [5.0, 0.0, -5.0], [-5.0, 0.0, -5.0], [0.0, 5.0, -5.0], [0.0, -5.0, -5.0],
    ])?;
    b.step("bevel", ChamferArgs::even(edges, 6f64.sqrt()))
}
