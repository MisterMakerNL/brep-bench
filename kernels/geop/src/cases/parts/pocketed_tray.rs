//! An open tray lying on its back, opening to −y: a block 72 × 16 × 48 sketched on the xz plane
//! and extruded 16 along +y; a pocket 66 × 42, 13 deep, sketched on the same plane and cut from
//! it; and four standoffs, each a ring of radii 5 and 2 sketched on a plane 5 along y and
//! extruded 8 up to the floor, joined to the tray (so each hole is blind, stopping on the floor).
use crate::shared::paths::offset_plane;
use crate::shared::{Drawing, extrude, made, sketch, zx, zx_at};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut block = Drawing::new();
    block.rectangle(zx_at(-36.0, -24.0), zx_at(36.0, 24.0));
    sketch(b, "block", zx(), block)?;
    extrude(b, "tray", "block", 16.0, Combine::NewBody)?;

    let mut pocket = Drawing::new();
    pocket.rectangle(zx_at(-33.0, -21.0), zx_at(33.0, 21.0));
    sketch(b, "pocket", zx(), pocket)?;
    extrude(b, "hollow", "pocket", 13.0, Combine::cut(made("extrude", "tray")))?;

    let tops = offset_plane(b, "tops", FrameAxis::Y, 5.0)?;
    let mut rings = Drawing::new();
    for (x, z) in [(26.0, 14.0), (-26.0, 14.0), (-26.0, -14.0), (26.0, -14.0)] {
        rings.circle(zx_at(x, z), 5.0);
        rings.circle(zx_at(x, z), 2.0);
    }
    sketch(b, "rings", tops, rings)?;
    let tray = b.solid()?;
    extrude(b, "standoffs", "rings", 8.0, Combine::join(tray))
}
