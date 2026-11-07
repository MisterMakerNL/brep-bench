//! A box from the origin to (a, b, c) with all twelve edges rounded to radius ρ, three rounds
//! meeting at each corner; turned by ψ about the axis (θ, φ).
//!
//! Refused, as `fillet/box-all-edges` is: `fillet::roll` rounds a box's four vertical edges but
//! refuses its top and bottom ones ("two canals meeting in a mitre are rung 3"; once the vertical
//! edges are rounded, "larger than `near` can hold"), and the ball's patch where three rounds meet
//! (`fillet::corner_among`, a recipe's lowered corner) takes its centre from the caller.
use crate::Params;
use cad_kernel::topo::Brep;

pub fn build(_: &Params) -> Result<Brep, String> {
    Err("no rounding of edges that meet at a vertex that the kernel places: fillet::roll rounds a \
         box's four vertical edges but refuses its top and bottom ones (\"two canals meeting in a \
         mitre are rung 3\"; once the vertical edges are rounded, \"larger than `near` can hold\"), \
         and the ball's patch where three rounds meet (fillet::corner_among, a recipe's lowered \
         corner) takes its centre from the caller"
        .into())
}
