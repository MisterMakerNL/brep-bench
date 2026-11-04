//! A box from the origin to (20, 10, 6), the four edges around its top face chamfered with
//! distance 1 across the top and 2 down the sides. The box is extruded and the four edges
//! chamfered in one step with two distances: geop measures the first into the face on the left
//! of each edge as it runs, seen from outside, the second into the face on its right; the top's
//! edges run round it so that the side is on their left and the top on their right.
use crate::shared::{Drawing, extrude, sketch, xy};
use crate::{Bench, Refusal};
use geop_ops_fillet::ChamferArgs;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut base = Drawing::new();
    base.rectangle([0.0, 0.0], [20.0, 10.0]);
    sketch(b, "base", xy(), base)?;
    extrude(b, "box", "base", 6.0, Combine::NewBody)?;
    let rim = b.edges(&[[10.0, 0.0, 6.0], [20.0, 5.0, 6.0], [10.0, 10.0, 6.0], [0.0, 5.0, 6.0]])?;
    b.step("bevel", ChamferArgs { distance2: Some(1.0.into()), ..ChamferArgs::even(rim, 2.0) })
}
