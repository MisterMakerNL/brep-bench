//! A thin blade from z = 0 to z = 60 whose section at every height is the rectangle 40 × 0.4
//! centered on the z axis, turned right-handed 6° a unit of height (a whole turn in all): its
//! corners run along four helices of radius √(20² + 0.2²) rising 60 a turn, and each wall is
//! swept by a horizontal segment between two of them.
//!
//! cadrs's solid Loft takes no guides, and its surface Loft through curves would need the
//! helices as curves of its own. Every section is the first one carried by one screw motion,
//! the one that carries each corner along its helix, so the blade is built as that: the
//! rectangle −20 ≤ x ≤ 20, −0.2 ≤ y ≤ 0.2 on the Top plane swept along the helical guide through
//! its corner (20, 0.2, 0), drawn as cadrs's Helix feature draws one, which the sweep follows by
//! the helix's screw motion.
use crate::bench::{p3, rectangle, sketch, xy};
use crate::shared::sweep::{helix, sweep};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let blade = sketch(xy(0.0), vec![rectangle(-20.0, -0.2, 20.0, 0.2)]);
    let (x, y) = (20.0f64, 0.2f64);
    let guide = helix(p3(0.0, 0.0, 0.0), x.hypot(y), 60.0, y.atan2(x), 1.0);
    sweep(k, &blade, guide)
}
