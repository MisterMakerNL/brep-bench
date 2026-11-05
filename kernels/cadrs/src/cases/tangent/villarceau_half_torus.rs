//! A solid torus about the z axis, its tube's middle circle of radius 5 and the tube of radius
//! 3, cut by the bitangent plane 3x − 4z = 0, keeping what lies on and below it (z ≤ 3x/4).
//!
//! The torus is the circle of radius 3 about (5, 0) on the Right plane (x = 0, sketch x along
//! +y) revolved a whole turn about z, so its seam lies in the plane x = 0 on the side y > 0
//! (the kernel has no torus primitive). What lies above the plane is cut away with a block: a
//! square of side 40 sketched on the plane itself, extruded 20 along its normal (−3, 0, 4)/5,
//! away from the kept side, reaching past the torus.
use crate::bench::{axis, cut, extrude, p3, plane, rectangle, revolve, round, sketch, v3, yz};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let tube = sketch(yz(0.0), vec![round((5.0, 0.0), 3.0)]);
    let ring = revolve(k, &tube, axis(p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0)), None)?;
    let on = plane(p3(0.0, 0.0, 0.0), v3(4.0, 0.0, 3.0), v3(-3.0, 0.0, 4.0));
    let above = extrude(k, &sketch(on, vec![rectangle(-20.0, -20.0, 20.0, 20.0)]), 20.0)?;
    cut(k, ring, above)
}
