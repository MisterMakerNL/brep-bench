//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a helical groove
//! ground by a wheel carried along a screw. The wheel: a torus, radii 6 and 2, centred at
//! (11.5, 0, 0), its axis (0, cos h, −sin h) with h = atan(2π·5.5 / 30). The screw: about the z
//! axis, right-handed, rising 30 a turn, from a turn of −108° to +348°. Everything the wheel
//! passes through, cut.
//!
//! Refused. kentron's one sweep that can follow a screw is a pipe along a helix in its Frenet
//! frame (`sweep::pipe_with`, `frenet`), and it takes no solid ("a pipe of a Solid"). Given the
//! wheel's shell it sweeps each face whole, which is what that face passes through only if the
//! motion never runs along it inside it; on a torus screwed about an axis it does not share, the
//! motion runs along it on its characteristic curve, none of the torus's circles. The groove's
//! wall is the envelope that curve sweeps out, which kentron has no operation to find, and
//! computing it here would be making the geometry outside the kernel.
use crate::Refusal;
use kentron::topology::{Model, Shape};

pub fn build(_m: &mut Model) -> Result<Shape, Refusal> {
    Err(Refusal::from(
        "kentron has no sweep of a solid along a screw motion: sweep::pipe along a helix takes no \
         solid, and swept face by face the torus's face would first have to be split along its \
         characteristic curve, none of its circles; the groove's wall is the envelope of the screwed \
         torus, which kentron has no operation to find",
    ))
}
