//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a helical V groove
//! ground by a wheel with a sharp 60° V rim (its axis through (14, 0, 0) leaning by the helix
//! angle) carried along the screw about the z axis that rises 30 a turn, from a turn of −144° to
//! +384°.
//!
//! The wheel is its triangle (tip 10 from its axis, the other corners 7.5 from it and 2.5 tan 30°
//! either side of its mid-plane) sketched on the plane through its axis and the point (4, 0, 0),
//! revolved. The groove is cut with cadrs's `Kernel::sweep_solid`: the wheel screwed about z
//! without end, rising 30 a turn, between z = −1 and z = 21. The case screws it from −144°
//! (wholly below the rod) to +384° (wholly above), so within the rod it passes every place the
//! endless screw would.
use crate::bench::{axis, cut, p3, plane, polygon, revolve, sketch, v3};
use crate::shared::swept::rod;
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, DefaultKernel, Kernel, SolidMotion};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let h = (std::f64::consts::TAU * 4.5 / 30.0).atan();
    let along = v3(0.0, h.cos(), -h.sin());
    let centre = p3(14.0, 0.0, 0.0);
    // The plane through the wheel's axis: sketch x from the axis toward the rod, y along it.
    let on = plane(centre, v3(-1.0, 0.0, 0.0), v3(-1.0, 0.0, 0.0).cross(&along));
    let w = 2.5 * 30f64.to_radians().tan();
    let section = sketch(on, vec![polygon(&[(10.0, 0.0), (7.5, w), (7.5, -w)])]);
    let wheel = revolve(k, &section, axis(centre, along), None)?;
    let axis = Axis { origin: p3(0.0, 0.0, 0.0), dir: crate::bench::unit(v3(0.0, 0.0, 1.0)) };
    let swept = k.sweep_solid(wheel, &SolidMotion::Screw { axis, rise: 30.0, from: -1.0, to: 21.0 })?;
    let swept = swept.bodies.first().copied().ok_or_else(|| Refusal::from("the sweep made no body"))?;
    let r = rod(k)?;
    cut(k, r, swept)
}
