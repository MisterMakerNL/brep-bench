//! A ball of radius 5 centered on the origin, its whole surface one rational B-spline surface,
//! the standard biquadratic sphere: a 9 × 5 net, the tensor product of the unit circle's nine
//! control points around the axis (weights 1, √2/2, …) and the semicircle's five from the south
//! pole to the north, whose first and last rows each collapse to a pole and whose edges u = 0
//! and u = 1 are the same meridian, its seam.
//!
//! cadrs makes no face from a bare surface (its `face_from_surface` takes a face of a body, and
//! putting the net in place of a ball's sphere with `replace_faces` fails), so the ball is made
//! and its surface converted: cadrs's sphere primitive, its poles on the z axis and its seam
//! toward +x, healed with `HealOp::ToBSpline`, which gives every curved surface as its exact
//! B-spline. The sphere's is the standard biquadratic net above: the same control points and
//! weights, its knots in radians (0 to 2π around, −π/2 to π/2 along the meridian) where the
//! description's run from 0 to 1. It is one face, closed on its seam, with a pole at each end.
use crate::bench::{ball, p3};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, HealOp, Kernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let b = ball(k, p3(0.0, 0.0, 0.0), 5.0)?;
    let r = k.heal(b, &[HealOp::ToBSpline])?;
    r.bodies.first().copied().ok_or_else(|| Refusal::from("the heal made no body"))
}
