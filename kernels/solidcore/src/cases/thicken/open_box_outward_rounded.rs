//! The floor and four walls of the cube from the origin to (10, 10, 10), with no top, an open
//! tray, thickened by 1 out of the cube, the gaps the moved faces leave at its eight edges closed
//! by quarter cylinders of radius 1 and at the floor's four corners by eighth balls.
use crate::bench::{edges_through_each, reloaded};
use crate::shared::thicken::open_box;
use crate::Refusal;
use solidcore_ops::{blend, thicken, Blend, Body, Corners};

pub fn build() -> Result<Body, Refusal> {
    let tray = open_box()?;
    // ops's thicken has no rounded corners (see `shared::thicken`): the tray thickened with its
    // moved faces extended to meet, then the eight corners that made rounded to radius 1, the
    // thickness, each about the tray's edge inside it; where three meet below the floor's
    // corners, the rounds close on the ball of radius 1 about the corner.
    thicken(&tray, 1.0, false)?;
    let (tray, order) = reloaded(&tray)?;
    let corners = edges_through_each(
        &tray,
        &order,
        &[
            // under the floor's four edges
            [5.0, -1.0, -1.0],
            [11.0, 5.0, -1.0],
            [5.0, 11.0, -1.0],
            [-1.0, 5.0, -1.0],
            // outside the four upright edges
            [-1.0, -1.0, 5.0],
            [11.0, -1.0, 5.0],
            [11.0, 11.0, 5.0],
            [-1.0, 11.0, 5.0],
        ],
    )?;
    blend(&tray, Blend::Round, &corners, &[1.0], &[], Corners::Filled)?;
    Ok(tray)
}
