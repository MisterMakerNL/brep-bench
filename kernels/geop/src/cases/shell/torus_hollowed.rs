//! A solid torus about the z axis, its tube of radius 2 about a middle circle of radius 6,
//! hollowed to a wall 0.5 thick with no opening: the torus less the torus of tube radius 1.5.
//! geop has no torus: it is turned from its circular section about the z axis, then shelled by
//! geop's shell with no face picked, which closes it all round.
use crate::shared::hollow::shell;
use crate::shared::{Drawing, axis, revolve, sketch, zx, zx_at};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut tube = Drawing::new();
    tube.circle(zx_at(6.0, 0.0), 2.0);
    sketch(b, "tube", zx(), tube)?;
    revolve(b, "ring", "tube", axis(FrameAxis::Z), 360.0, Combine::NewBody)?;
    shell(b, "hollow", &[], 0.5, false)
}
