//! A gear blank with one tooth space cut by a rack tooth rolled round it (module 2, 20 teeth, 20°
//! pressure angle). The blank: the disc of radius 22 about the y axis from y = 0 to y = 6, pitch
//! circle 20. The tooth: the prism from y = −2 to y = 8 on the trapezoid with corners
//! (18, ±0.842856) and (24, ±3.026666) in (x, z). The rack's pitch line x = 20 rolls without
//! slipping on the pitch circle from 60° on one side of (20, 0, 0) to 60° on the other; cut from
//! the blank everything the tooth passes through.
//!
//! Refused. kentron's sweeps slide (`sweep::extrude`), turn about one axis (`sweep::revolve`) or
//! carry a profile along a spine in a frame that follows the spine (`sweep::pipe`, `pipe_with`),
//! and none takes a solid. Seen from the blank, the rolling rack does none of these: its frame
//! would have to follow an involute of the pitch circle, with a cusp where the tooth's middle
//! touches, a path that exists only if the implementation builds the involute from its formula.
//! And swept face by face, each flat flank would first have to be split along the line where the
//! roll runs along it, which moves across the flank: the space's flanks and root are the envelope
//! of the tooth, which kentron has no operation to find.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "kentron has no sweep of a solid along a rolling motion: it slides, turns about one axis, or \
         follows a spine it is given, and the rack's path seen from the blank is an involute the \
         implementation would have to build from its formula; the space's involute flanks and \
         trochoid root are the envelope of the tooth, which kentron has no operation to find",
    ))
}
