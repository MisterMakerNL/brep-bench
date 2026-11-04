//! A cube of side 10, from the origin to (10, 10, 10), with all twelve of its edges rounded to
//! radius 1, three rounds meeting at each corner.
use crate::bench::{all_edges, reloaded};
use crate::Refusal;
use solidcore_ops::{blend, make_box, Blend, Body, Corners};

pub fn build() -> Result<Body, Refusal> {
    let (cube, order) = reloaded(&make_box([0.0, 0.0, 0.0], [10.0, 10.0, 10.0])?)?;
    blend(&cube, Blend::Round, &all_edges(&order), &[1.0], &[], Corners::Filled)?;
    Ok(cube)
}
