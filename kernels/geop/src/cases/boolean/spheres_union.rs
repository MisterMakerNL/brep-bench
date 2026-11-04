//! Two balls of radius 5, one centred on the origin and one on (6, 0, 0). Make their union. geop has no
//! ball of its own: each is a half disc in the xz plane turned about the vertical through its
//! centre; then the boolean union of the two.
use crate::shared::booleans::{ball, boolean};
use crate::{Bench, Refusal};
use geop_ops_model::BooleanOp;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let first = ball(b, "first", [0.0, 0.0, 0.0], 5.0)?;
    let second = ball(b, "second", [6.0, 0.0, 0.0], 5.0)?;
    boolean(b, "union", &first, &second, BooleanOp::Union)
}
