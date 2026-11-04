//! What the solidcore implementations of the offset cases share: a solid's offset with its
//! convex edges rounded, the kernel's own offset (`SmApiOffsetBrepFull`) with its option to
//! round them.
//!
//! solidcore-ops offers a solid's offset as [`solidcore_ops::offset_sheet`], which moves every
//! face onto its surface's offset and extends the moved faces until they meet
//! (`bDoExtendedOffset` on): the offset with its edges kept sharp. The kernel's offset also
//! rounds them, joining the moved faces at each edge where they part by the offset of the edge
//! itself (a cylinder or torus about it) and at each corner by the offset of the corner (a
//! ball), which is the solid of every point within the distance of the body. Ops does not pass
//! that option on, so a grown solid is made by the kernel itself, on the body's record made its
//! own ([`in_kernel`]).
use crate::Refusal;
use solidcore::offset::offset_executive::p2::{offset_brep_simple, OffsetOptions};
use solidcore::topology::brep::is_outward_solid;
use solidcore::{Brep, Kernel};
use solidcore_convert::brep::{to_kernel, to_record, Order};
use solidcore_ops::Body;

/// `op` run by the kernel on `body`'s record made its own again (with the record's [`Order`]),
/// and what it makes taken back as a body. The kernel runs with the fixes ops's session runs
/// with, and the result is held to ops's contract for its offsets: it has faces, and a solid
/// made of a solid is one (closed, manifold, valid and facing out: the kernel's
/// `is_outward_solid`).
pub fn in_kernel(
    body: &Body,
    op: impl FnOnce(&mut Kernel, &mut Brep, &Order) -> solidcore::Result<Brep>,
) -> Result<Body, Refusal> {
    let (record, _) = body.to_record()?;
    let mut k = Kernel::default();
    for name in solidcore_ops::session::fixes() {
        if let Some(on) = k.fixes.flag_mut(name) {
            *on = true;
        }
    }
    let (mut brep, order) =
        to_kernel(&mut k, &record, true).map_err(|e| format!("the record does not convert: {e:?}"))?;
    let solid = is_outward_solid(&mut k, &mut brep)?;
    let mut made = op(&mut k, &mut brep, &order)?;
    if made.face_list().is_empty() {
        return Err("the result is empty".into());
    }
    if solid && !is_outward_solid(&mut k, &mut made)? {
        return Err("the result is not a closed solid".into());
    }
    let (record, _) =
        to_record(&mut k, &mut made).map_err(|e| format!("the result makes no record: {e:?}"))?;
    Ok(Body::from_record(&record, true)?.0)
}

/// The kernel's offset of the solid `body` by `distance` (negative: inward), its convex edges
/// rounded (`bDoExtendedOffset` off) and the moved faces that cross trimmed and merged
/// (`bDoSelfInt` on): every point within `distance` of the body, or, inward, every point at
/// least `-distance` from its outside.
pub fn offset_rounded(body: &Body, distance: f64) -> Result<Body, Refusal> {
    in_kernel(body, |k, b, _| {
        offset_brep_simple(
            k,
            b,
            OffsetOptions { distance, extend_convex_edges: false, merge_results: true },
        )
    })
}
