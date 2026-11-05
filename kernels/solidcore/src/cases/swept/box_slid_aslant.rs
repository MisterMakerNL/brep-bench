//! The solid the box from the origin to (10, 6, 4) fills as it slides without turning along
//! (8, 5, 3), to the box from (8, 5, 3) to (18, 11, 7).
//!
//! solidcore has no sweep of a solid, so the solid is built as what it is: the box where it
//! starts, and what each of its three faces facing along the slide (x = 10, y = 6, z = 4) sweeps
//! as it slides, the face swept along the slide without turning, an oblique prism. Every point
//! the box passes leaves it, going along the slide, through one of those faces.
use crate::bench::{fuse_all, line, polygon};
use crate::Refusal;
use solidcore_ops::sweep::{sweep_placed, SweepFrame};
use solidcore_ops::{make_box, Body};

const SLIDE: [f64; 3] = [8.0, 5.0, 3.0];

/// The face of `corners` swept along the slide, its path from the face's first corner.
fn slid(corners: [[f64; 3]; 4]) -> Result<Body, Refusal> {
    let [x, y, z] = corners[0];
    let path = line([x, y, z], [x + SLIDE[0], y + SLIDE[1], z + SLIDE[2]]);
    let (body, _, _) = sweep_placed(&polygon(&corners), &[path], SweepFrame::Translate, true)?;
    Ok(body)
}

pub fn build() -> Result<Body, Refusal> {
    let start = make_box([0.0; 3], [10.0, 6.0, 4.0])?;
    let x = slid([[10.0, 0.0, 0.0], [10.0, 6.0, 0.0], [10.0, 6.0, 4.0], [10.0, 0.0, 4.0]])?;
    let y = slid([[0.0, 6.0, 0.0], [0.0, 6.0, 4.0], [10.0, 6.0, 4.0], [10.0, 6.0, 0.0]])?;
    let z = slid([[0.0, 0.0, 4.0], [10.0, 0.0, 4.0], [10.0, 6.0, 4.0], [0.0, 6.0, 4.0]])?;
    fuse_all([start, x, y, z])
}
