//! A rod, the cylinder of radius 5 about the z axis from z = 0 to z = 20, with a helical groove
//! cut by a ball of radius 1 whose center follows the helix of radius 5 about the z axis from
//! (5, 0, −2), right-handed, rising 4 a turn, six whole turns to (5, 0, 22).
//!
//! cadrs has no sweep of a solid, so the ball's sweep is built as what it is: the tube about
//! its center's path, the disk of radius 1 square to the helix where it starts swept along the
//! helix (drawn as cadrs's Helix feature draws one, which the sweep follows by its screw
//! motion), and the ball at each end, whose half beyond the tube closes it (both clear of the
//! rod). Each ball is cadrs's sphere primitive.
use crate::bench::{ball, cut, p3, round, sketch, square_to, union_each};
use crate::shared::sweep::{helix, helix_at, sweep};
use crate::shared::swept::rod;
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let body = rod(k)?;
    let o = p3(0.0, 0.0, -2.0);
    let (start, tangent) = helix_at(o, 5.0, 4.0, 0.0, 0.0);
    let disk = sketch(square_to(start, tangent), vec![round((0.0, 0.0), 1.0)]);
    let tube = sweep(k, &disk, helix(o, 5.0, 4.0, 0.0, 6.0))?;
    let first = ball(k, p3(5.0, 0.0, -2.0), 1.0)?;
    let last = ball(k, p3(5.0, 0.0, 22.0), 1.0)?;
    let groove = union_each(k, tube, &[first, last])?;
    cut(k, body, groove)
}
