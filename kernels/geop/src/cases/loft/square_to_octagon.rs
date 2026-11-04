//! A transition piece lofted from the square −20 ≤ x, y ≤ 20 on the xy plane to the octagon (the
//! square 28 × 28 with its corners cut off) on a plane 30 up, each corner of the square joined to
//! a whole cut side of the octagon by a flat triangle: the two sketched and lofted.
use crate::shared::paths::{loft, offset_plane};
use crate::shared::{Drawing, sketch, xy};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut square = Drawing::new();
    square.rectangle([-20.0, -20.0], [20.0, 20.0]);
    sketch(b, "square", xy(), square)?;
    let top = offset_plane(b, "top", FrameAxis::Z, 30.0)?;
    let mut octagon = Drawing::new();
    octagon.polygon(&[
        [6.0, -14.0],
        [14.0, -6.0],
        [14.0, 6.0],
        [6.0, 14.0],
        [-6.0, 14.0],
        [-14.0, 6.0],
        [-14.0, -6.0],
        [-6.0, -14.0],
    ]);
    sketch(b, "octagon", top, octagon)?;
    loft(b, "hood", &["square", "octagon"], Vec::new())
}
