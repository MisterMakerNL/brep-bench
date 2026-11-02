//! The solid a torus fills as it tumbles: the torus centred at (3, 0, 2), axis parallel to the
//! z axis, radii 1 and 0.5, turned by t/4 about the vertical line through (2, 0, 0) and then by
//! −t about the line through (0, 0.5, 0.5) parallel to the x axis, for t from −75° to 75°.
//!
//! Refused. The torus's axis of turning changes from instant to instant. kentron's sweeps slide
//! (`sweep::extrude`), turn about one fixed axis (`sweep::revolve`) or carry a profile along a
//! spine in a frame that follows the spine (`sweep::pipe`, `pipe_with`), none takes a solid, and
//! none of their frames turns a shape like this along its centre's path. The solid's boundary is
//! the envelope of the torus under the motion, along characteristic curves that are none of its
//! circles, which kentron has no operation to find.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "kentron has no sweep of a solid along a pair of turns about skew axes: it slides, turns \
         about one axis, or follows a spine in the spine's frame, none of which is this tumble; the \
         boundary is the envelope of the tumbling torus, which kentron has no operation to find",
    ))
}
