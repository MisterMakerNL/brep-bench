//! A solid duct along the y axis from y = 0 to y = 40, from the square A (10, 0, −10),
//! B (−10, 0, −10), C (−10, 0, 10), D (10, 0, 10) to the diamond A′ (0, 40, −12), B′ (−12, 40, 0),
//! C′ (0, 40, 12), D′ (12, 40, 0), each corner running straight to the corner of the same letter:
//! the square sketched on the xz plane, the diamond on a plane 40 along y, and lofted between
//! them — two sections, joined by ruled walls — with A, B, C, D matched to A′, B′, C′, D′.
use crate::shared::paths::offset_plane;
use crate::shared::{Drawing, sketch, zx, zx_at};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_extrude_revolve::{LoftArgs, Output};
use geop_ops_model::Combine;
use geop_ops_model::sketch::{sketch_element, whole_sketch};

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut square = Drawing::new();
    let (square_corners, _) =
        square.polygon(&[(10.0, -10.0), (-10.0, -10.0), (-10.0, 10.0), (10.0, 10.0)].map(|(x, z)| zx_at(x, z)));
    sketch(b, "square", zx(), square)?;

    let far = offset_plane(b, "far", FrameAxis::Y, 40.0)?;
    let mut diamond = Drawing::new();
    let (diamond_corners, _) =
        diamond.polygon(&[(0.0, -12.0), (-12.0, 0.0), (0.0, 12.0), (12.0, 0.0)].map(|(x, z)| zx_at(x, z)));
    sketch(b, "diamond", far, diamond)?;

    let matches = (square_corners.iter().map(|&p| sketch_element("square", p)))
        .chain(diamond_corners.iter().map(|&p| sketch_element("diamond", p)))
        .collect();
    b.step(
        "duct",
        LoftArgs {
            profiles: vec![whole_sketch("square"), whole_sketch("diamond")],
            matches,
            guides: Vec::new(),
            output: Output::Solid(Combine::NewBody),
        },
    )
}
