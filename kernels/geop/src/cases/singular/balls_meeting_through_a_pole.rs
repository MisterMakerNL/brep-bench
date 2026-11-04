//! Two balls of radius 5, one centred on the origin and one on (4.8, 0, 3.6), united; the circle
//! where their spheres meet passes through a pole of each. geop has no ball of its own: each is
//! a half disc in the xz plane turned about the vertical through its centre, so its poles are on
//! that vertical. Then the boolean union.
use crate::shared::booleans::{ball, boolean};
use crate::{Bench, Refusal};
use geop_ops_model::BooleanOp;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let first = ball(b, "first", [0.0, 0.0, 0.0], 5.0)?;
    let second = ball(b, "second", [4.8, 0.0, 3.6], 5.0)?;
    boolean(b, "union", &first, &second, BooleanOp::Union)
}
