//! A cube of side 10 from the origin to (10, 10, 10), all twelve edges chamfered with distance 2
//! on both faces, one after another, each its own chamfer ending on the faces at its ends: the
//! three chamfers at each corner mitre, meeting in a point.
use crate::shared::chamfer::chamfer_one_by_one;
use crate::Refusal;
use solidcore_ops::{make_box, Body};

pub fn build() -> Result<Body, Refusal> {
    let cube = make_box([0.0, 0.0, 0.0], [10.0, 10.0, 10.0])?;
    // the middle of each edge, which each chamfer before it leaves on it
    let mut middles = Vec::new();
    for u in [0.0, 10.0] {
        for w in [0.0, 10.0] {
            middles.extend([[5.0, u, w], [u, 5.0, w], [u, w, 5.0]]);
        }
    }
    chamfer_one_by_one(cube, &middles, 2.0)
}
