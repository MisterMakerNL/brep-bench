//! A cube of side 10 between the origin and (10, 10, 10), all twelve edges chamfered with
//! distance 2 on both faces, the corners mitred: the cube extruded, then one chamfer of all twelve
//! edges, each picked by its midpoint. geop's chamfer bevels the edges one after the other, each
//! tool built from the cube, and leaves each corner to its three chamfers, which cross in a point.
use crate::shared::{Drawing, extrude, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_fillet::ChamferArgs;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([0.0, 0.0], [10.0, 10.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "cube", "base", 10.0, Combine::NewBody)?;
    let mut midpoints = Vec::new();
    for a in [0.0, 10.0] {
        for c in [0.0, 10.0] {
            midpoints.extend([[5.0, a, c], [a, 5.0, c], [a, c, 5.0]]);
        }
    }
    let edges = b.edges(&midpoints)?;
    b.step("bevel", ChamferArgs::even(edges, 2.0))
}
