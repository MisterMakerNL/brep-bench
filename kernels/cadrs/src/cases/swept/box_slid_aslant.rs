//! The solid the box from (0, 0, 0) to (10, 6, 4) fills as it slides without turning along
//! (8, 5, 3), to the box from (8, 5, 3) to (18, 11, 7).
//!
//! cadrs has no sweep of a solid, so the solid is built as what it is: the box where it starts,
//! and what each of its three faces facing along the slide (x = 10, y = 6, z = 4) sweeps as it
//! slides, the face's rectangle sketched on its plane and extruded along the slide, an oblique
//! prism. Every point the box passes leaves it, going along the slide, through one of those
//! faces (the box where it ends lies in their union).
use crate::bench::{block, extrude_along, p3, rectangle, sketch, union_each, v3, xy, xz, yz};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let slide = v3(8.0, 5.0, 3.0);
    let start = block(k, p3(0.0, 0.0, 0.0), p3(10.0, 6.0, 4.0))?;
    // the faces x = 10 (sketch (y, z)), y = 6 (sketch (x, z)) and z = 4 (sketch (x, y))
    let faces = [
        sketch(yz(10.0), vec![rectangle(0.0, 0.0, 6.0, 4.0)]),
        sketch(xz(6.0), vec![rectangle(0.0, 0.0, 10.0, 4.0)]),
        sketch(xy(4.0), vec![rectangle(0.0, 0.0, 10.0, 6.0)]),
    ];
    let mut slid = Vec::new();
    for face in &faces {
        slid.push(extrude_along(k, face, slide, slide.norm())?);
    }
    union_each(k, start, &slid)
}
