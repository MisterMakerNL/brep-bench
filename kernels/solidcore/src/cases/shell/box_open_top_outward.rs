//! The box from the origin to (10, 8, 6) shelled outward to a wall 1 thick, open at its top face,
//! z = 6, the wall every point outside the box within 1 of its other faces: the box grown by 1 and
//! cut off at z = 6, less the box. Its outside is rounded at the box's convex edges and corners.
//!
//! The kernel's shell refuses to round an outward wall (`SmApiShellBrepFull` with convex edges
//! rounded, `bDoExtendedOffset` off, rejects the box as invalid input), and ops's shell keeps the
//! edges sharp. So the cup is made as the description composes it: the kernel's offset of the
//! box, its convex edges rounded, cut off at the plane of the top and hollowed by the box.
use crate::bench::cut_all;
use crate::shared::offset::offset_rounded;
use crate::Refusal;
use solidcore_ops::{make_box, Body};

pub fn build() -> Result<Body, Refusal> {
    let block = make_box([0.0, 0.0, 0.0], [10.0, 8.0, 6.0])?;
    let grown = offset_rounded(&block, 1.0)?;
    let above = make_box([-2.0, -2.0, 6.0], [14.0, 12.0, 2.0])?;
    cut_all(grown, [above, block])
}
