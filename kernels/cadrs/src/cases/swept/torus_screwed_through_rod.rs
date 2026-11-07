//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a helical groove
//! ground by a torus (radii 6 and 2, centered (11.5, 0, 0), its axis leaning by the helix angle)
//! carried along the screw about the z axis that rises 30 a turn, from a turn of −108° to +348°.
//!
//! The groove is cut with cadrs's `Kernel::sweep_solid`: the torus screwed about z without end,
//! rising 30 a turn, between the planes z = −1 and z = 21 (the region it covers seen along the
//! screw's helices, swept up the screw). The case screws it from −108° (wholly below the rod) to
//! +348° (wholly above), so within the rod it passes every place the endless screw would.
use crate::bench::{cut, p3, torus, v3};
use crate::shared::swept::rod;
use crate::Refusal;
use cadrs_kernel::{Axis, BodyId, DefaultKernel, Kernel, SolidMotion};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let h = (std::f64::consts::TAU * 5.5 / 30.0).atan();
    let wheel = torus(k, p3(11.5, 0.0, 0.0), v3(0.0, h.cos(), -h.sin()), 6.0, 2.0)?;
    let axis = Axis { origin: p3(0.0, 0.0, 0.0), dir: crate::bench::unit(v3(0.0, 0.0, 1.0)) };
    let swept = k.sweep_solid(wheel, &SolidMotion::Screw { axis, rise: 30.0, from: -1.0, to: 21.0 })?;
    let swept = swept.bodies.first().copied().ok_or_else(|| Refusal::from("the sweep made no body"))?;
    let r = rod(k)?;
    cut(k, r, swept)
}
