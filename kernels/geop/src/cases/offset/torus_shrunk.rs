//! A solid torus about the z axis, its tube of radius 3 about a middle circle of radius 10,
//! shrunk by 1.5: the torus of tube radius 1.5 about the same circle. geop has no torus: it is
//! turned from its circular section about the z axis, and its face offset 1.5 into it by geop's
//! face offset.
use crate::shared::hollow::{offset_faces, on_torus};
use crate::shared::{Drawing, axis, revolve, sketch, zx, zx_at};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut tube = Drawing::new();
    tube.circle(zx_at(10.0, 0.0), 3.0);
    sketch(b, "tube", zx(), tube)?;
    revolve(b, "ring", "tube", axis(FrameAxis::Z), 360.0, Combine::NewBody)?;
    // One piece of the torus picked: the face offset takes along the faces it runs smoothly
    // into, the rest of the torus.
    let on = on_torus(10.0, 3.0, 22.5, 45.0);
    offset_faces(b, "shrink", &[on], -1.5)
}
