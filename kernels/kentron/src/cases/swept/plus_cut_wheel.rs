//! A wheel cut by a plus-shaped tool turning in mesh with it, as a gear shaper cuts. The tool: a
//! plus centred on the origin, arms 30 long and 8 wide along ±x and ±y, extruded from z = −10 to
//! z = 0. The blank: the disc of radius 40 about the vertical line through (−50, 0), from z = −8
//! to z = −2. As the tool turns by t about the z axis the wheel turns by −2t/3 about its own axis,
//! for t from 0° to 540°; cut from the wheel everything the tool passes through, seen from it.
//!
//! Refused. Seen from the wheel, the tool turns about its own axis while that axis is carried
//! round the wheel's: two turns at once, in the ratio 5 : 2. kentron's sweeps slide
//! (`sweep::extrude`), turn about one axis (`sweep::revolve`) or carry a profile along a spine in
//! a frame that follows the spine (`sweep::pipe`, `pipe_with`), and none takes a solid; along the
//! circle the tool's centre runs on, the tool turns 5/2 as fast as that circle's own frame, which
//! none of them does. Swept face by face, each flat side of the plus would first have to be split
//! where the motion runs along it, a line that moves across it: the wheel's walls are the
//! envelope of the plus, which kentron has no operation to find.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "kentron has no sweep of a solid along a pair of turns: it slides, turns about one axis, or \
         follows a spine in the spine's frame, and the plus seen from the wheel turns 5/2 as fast as \
         the circle its centre runs on; the wheel's walls are the envelope of the plus, which kentron \
         has no operation to find",
    ))
}
