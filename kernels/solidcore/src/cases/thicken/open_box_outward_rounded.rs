//! The tray, five faces of the cube from the origin to (10, 10, 10) open at the top, thickened by
//! 1 out of the cube, the gaps the moved faces leave at the tray's eight edges closed by quarter
//! cylinders of radius 1 and at the floor's four corners by eighth balls.
//!
//! The kernel's shell refuses to round the moved faces where they part (`SmApiShellBrepFull`
//! with `bDoExtendedOffset` off rejects the tray as invalid input), so the tray is thickened
//! with the moved faces extended to meet, and the eight outer edges where they do (the floor's
//! four at z = −1 and the four upright ones) are rounded to radius 1, three rounds meeting at
//! each of the floor's corners: each round's axis is the tray's edge, 1 from both moved faces,
//! and each corner's ball is centred on the tray's corner.
use crate::shared::thicken::{rounded_at, thickened, tray};
use crate::Refusal;
use solidcore_ops::Body;

pub fn build() -> Result<Body, Refusal> {
    // the tray's faces face out of the cube: along them
    let sharp = thickened(tray()?, 1.0, false)?;
    let edges = [
        [5.0, -1.0, -1.0],
        [11.0, 5.0, -1.0],
        [5.0, 11.0, -1.0],
        [-1.0, 5.0, -1.0],
        [-1.0, -1.0, 5.0],
        [11.0, -1.0, 5.0],
        [11.0, 11.0, 5.0],
        [-1.0, 11.0, 5.0],
    ];
    rounded_at(&sharp, &edges, 1.0)
}
