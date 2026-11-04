//! What the solidcore implementations of the shell cases share: the kernel's shell with the
//! walls rounded where moved faces part.
//!
//! solidcore-ops's [`solidcore_ops::shell`] is the kernel's `SmApiShellBrepFull` with the moved
//! faces extended until they meet at every edge where they part (`bDoExtendedOffset` on), so
//! its walls stay sharp: at a concave edge of an inward wall, at a convex edge of an outward
//! one. The kernel's shell can also round them, joining the moved faces there by the offset of
//! the edge itself (a cylinder about it) and of a corner (a ball), making the wall every point
//! within its thickness of the faces. Ops does not pass that option on, so a rounded wall is
//! made by the kernel itself, on the body's record ([`crate::shared::offset::in_kernel`]).
use crate::bench::{cut, edges_through_each, face_through, order, reloaded};
use crate::shared::offset::in_kernel;
use crate::Refusal;
use solidcore::offset::offset_executive::p2::{shell_brep_simple, OffsetOptions, ShellOptions};
use solidcore_ops::{blend, make_box, make_sphere, shell, Blend, Body, Corners};

/// The part of the ball of radius 5 centred on the origin below the plane z = `height`, shelled
/// inward to a wall 1 thick, open at its flat face: the `sphere-open*` and `sphere-pinhole*`
/// cases.
pub fn ball_below_shelled_open(height: f64) -> Result<Body, Refusal> {
    let ball = make_sphere([0.0, 0.0, 0.0], 5.0)?;
    let above = make_box([-6.0, -6.0, height], [12.0, 12.0, 6.0])?;
    let part = cut(ball, above)?;
    let flat = face_through(&part, [0.0, 0.0, height])?;
    shell(&part, &order(&part)?, &[flat], -1.0)?;
    Ok(part)
}

/// The box from the origin to (20, 14, 10) with its four vertical edges rounded to radius 2: the
/// plate of the `round-*` and `rounded-tray` cases.
pub fn rounded_plate() -> Result<Body, Refusal> {
    let (plate, order) = reloaded(&make_box([0.0, 0.0, 0.0], [20.0, 14.0, 10.0])?)?;
    let corners = [[0.0, 0.0, 5.0], [20.0, 0.0, 5.0], [20.0, 14.0, 5.0], [0.0, 14.0, 5.0]];
    let edges = edges_through_each(&plate, &order, &corners)?;
    blend(&plate, Blend::Round, &edges, &[2.0], &[], Corners::Filled)?;
    Ok(plate)
}

/// `body` shelled to a wall `thickness` thick (negative: inward), open at the faces of its
/// record indices `open`, the moved faces joined by rounds where they part: `SmApiShellBrepFull`
/// as ops calls it, but with `bDoExtendedOffset` off.
pub fn shell_rounded(body: &Body, open: &[usize], thickness: f64) -> Result<Body, Refusal> {
    in_kernel(body, |k, b, order| {
        let open: Vec<_> = open.iter().map(|&i| order.faces[i]).collect();
        let options = ShellOptions {
            offset: OffsetOptions { distance: thickness, extend_convex_edges: false, merge_results: true },
            create_offset_solid: true,
            merge_sheet_side_walls: true,
        };
        shell_brep_simple(k, b, options, &open)
    })
}
