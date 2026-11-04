//! A solid torus about the z axis, its tube of radius 2 about a middle circle of radius 4, grown
//! by 3: every point within 5 of the middle circle, an apple whose face crosses the axis in two
//! dimples. geop has no torus: it is turned from its circular section about the z axis, and its
//! face offset 3 out of it by geop's face offset.
use crate::shared::hollow::{offset_faces, on_torus};
use crate::shared::{Drawing, axis, revolve, sketch, zx, zx_at};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut tube = Drawing::new();
    tube.circle(zx_at(4.0, 0.0), 2.0);
    sketch(b, "tube", zx(), tube)?;
    revolve(b, "ring", "tube", axis(FrameAxis::Z), 360.0, Combine::NewBody)?;
    // One piece of the torus picked: the face offset takes along the faces it runs smoothly
    // into, the rest of the torus.
    let on = on_torus(4.0, 2.0, 22.5, 45.0);
    offset_faces(b, "grow", &[on], 3.0)
}
