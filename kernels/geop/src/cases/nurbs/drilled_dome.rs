//! The block under the bicubic Bézier dome of `nurbs/bezier-dome`, drilled through by a vertical
//! hole of radius 3. Refused: geop has no surface given by its control net. Its sketches take any
//! NURBS curve, but its surfaces come only from operations — extrude, revolve, sweep, a loft or
//! network surface that interpolates curves, a ruled or Coons boundary surface — and none of them
//! makes a general tensor-product patch from given control points.
use crate::{Bench, Refusal};

pub fn build(_: &mut Bench) -> Result<(), Refusal> {
    Err("geop has no surface given by its control net: a sketch takes any NURBS curve, but its surfaces come only from operations (extrude, revolve, sweep, a loft or network surface interpolating curves, a ruled or Coons boundary surface), none of which makes a general tensor-product patch from given control points".into())
}
