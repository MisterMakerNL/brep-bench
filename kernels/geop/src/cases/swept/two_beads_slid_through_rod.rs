//! A rod, the cylinder of radius 5 about the z axis from z = 0 to 20, grooved by a wheel of two
//! beads slid 40 along z, from wholly below the rod to wholly above it: two tori on one axis
//! along y, the first centred (11.5, 0, −9), tube radius 2, the second (11.5, 1.5, −9), tube
//! radius 1.5, each with its tube's centres on a circle of radius 6.
//!
//! geop sweeps a profile, not a solid, so what the wheel passes through is built from what it is.
//! Sliding along z, each bead fills at each height everything that some height of it covers:
//! wherever all of it has passed (from z = −1 to z = 21 for both, which takes in the rod), that is
//! its shadow along z, the slot between the circles of its tube's radius about (5.5, y) and
//! (17.5, y) and the lines tangent to both. So each bead's slot, sketched on a plane at z = −1 and
//! extruded 22, to z = 21, is cut from the rod.
use crate::shared::paths::{offset_plane, slot};
use crate::shared::{Drawing, extrude, sketch, xy};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_model::Combine;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut end = Drawing::new();
    end.circle([0.0, 0.0], 5.0);
    sketch(b, "end", xy(), end)?;
    extrude(b, "rod", "end", 20.0, Combine::NewBody)?;

    let below = offset_plane(b, "below", FrameAxis::Z, -1.0)?;
    for (id, y, radius) in [("first_bead", 0.0, 2.0), ("second_bead", 1.5, 1.5)] {
        let shadow = format!("{id}_shadow");
        sketch(b, &shadow, below.clone(), slot(y, radius))?;
        let rod = b.solid()?;
        extrude(b, id, &shadow, 22.0, Combine::cut(rod))?;
    }
    Ok(())
}
