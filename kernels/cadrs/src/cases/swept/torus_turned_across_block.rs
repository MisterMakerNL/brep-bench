//! A sector of a ring about the z axis (radii 8 to 12, z from −1 to 1, polar angle 0° to 60°),
//! grooved by a torus (center (5, 0, 0), radii 2.5 and 2, its axis leaning 25° from z toward +y)
//! turned rigidly about the z axis from −140° to +140°.
//!
//! The block is its section (r from 8 to 12, z from −1 to 1) revolved 60° about z. The groove is
//! cut with cadrs's `Kernel::sweep_solid`, the torus turned a whole turn about z (the solid of
//! revolution it generates). The case turns it from −140° to +140° only; that cuts the block the
//! same, since the torus's points that reach the block (8 or more from the axis) all lie within
//! 30° of +x, so every point of the block the whole turn would sweep is swept within ±(60° + 30°),
//! inside the case's turn.
use crate::bench::{axis, cut, p3, rectangle, revolve, sketch, torus, v3, xz};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel, Kernel, SolidMotion};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let z = axis(p3(0.0, 0.0, 0.0), v3(0.0, 0.0, 1.0));
    let ring = sketch(xz(0.0), vec![rectangle(8.0, -1.0, 12.0, 1.0)]);
    let block = revolve(k, &ring, z, Some(60.0))?;
    let (s, c) = 25f64.to_radians().sin_cos();
    let cutter = torus(k, p3(5.0, 0.0, 0.0), v3(0.0, s, c), 2.5, 2.0)?;
    let swept = k.sweep_solid(cutter, &SolidMotion::Turn { axis: z })?;
    let swept = swept.bodies.first().copied().ok_or_else(|| Refusal::from("the sweep made no body"))?;
    cut(k, block, swept)
}
