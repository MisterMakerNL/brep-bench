//! A rod, the cylinder of radius 5 about the z axis from z = 0 to 20, grooved by a torus (axis
//! along y, tube centres on a circle of radius 6 about (11.5, 0, z), tube radius 2) slid along z
//! from its centre at z = −9, wholly below the rod, to z = 29, wholly above it.
//!
//! geop sweeps a profile, not a solid, so what the torus passes through is built from what it is.
//! Sliding along z, it fills at each height everything that some height of it covers: wherever
//! all of it has passed (from z = −1, 8 above its start, to z = 21, 8 below its end, which takes
//! in the rod), that is its shadow along z, the slot between the circles of radius 2 about
//! (5.5, 0) and (17.5, 0) — its tube's sections in its mid-plane — and the lines y = ±2 tangent to
//! both. So the slot, sketched on a plane at z = −1 and extruded 22, to z = 21, is cut from the
//! rod.
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
    sketch(b, "shadow", below, slot(0.0, 2.0))?;
    let rod = b.solid()?;
    extrude(b, "groove", "shadow", 22.0, Combine::cut(rod))
}
