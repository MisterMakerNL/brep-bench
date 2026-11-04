//! The block under the bicubic Bézier dome of `nurbs/bezier-dome`, with everything above the
//! plane z = 4 + x/2 cut away: the block split by that plane, and the part below it kept.
use crate::shared::nurbs::{bezier_dome, split_keeping};
use crate::Refusal;
use solidcore_ops::split::Tool;
use solidcore_ops::Body;
use solidcore_record::brep::Surface;

pub fn build() -> Result<Body, Refusal> {
    // the plane through (0, 0, 4), along y and along (1, 0, 1/2), reaching past the block
    let plane = Surface::Plane { origin: [0.0, 0.0, 4.0], u_axis: [0.0, 1.0, 0.0], v_axis: [1.0, 0.0, 0.5] };
    let tool = Tool { surface: plane, domain: [[-10.0, 30.0], [-10.0, 30.0]] };
    // the part below it, which holds (10, 10, 1)
    split_keeping(&bezier_dome()?, &tool, [10.0, 10.0, 1.0])
}
