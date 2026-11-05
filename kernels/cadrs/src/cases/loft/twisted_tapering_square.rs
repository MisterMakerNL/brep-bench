//! A square column from z = 0 to z = 40 whose section at height z is the square of side
//! 30 − z/2 centered on the z axis, turned 4.5° a unit of height: its corners run along four
//! conical spirals, and each wall is swept by a horizontal segment between two of them.
//!
//! Refused. cadrs's solid Loft takes no guides and fits its own surface through its sections,
//! its surface Loft through curves needs the four spirals as curves, and the kernel has no
//! conical spiral (its Part Studio's Helix on a cone hands it quintic Bézier pieces only near
//! the spiral, geometry made outside the kernel); its sweep carries a profile rigidly, with no
//! law to scale it as it turns.
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(_k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    Err(Refusal::from(
        "cadrs has no conical spiral curve to guide a loft along (its solid Loft takes no \
         guides), and its sweep carries a profile rigidly, with no law to scale it as it turns",
    ))
}
