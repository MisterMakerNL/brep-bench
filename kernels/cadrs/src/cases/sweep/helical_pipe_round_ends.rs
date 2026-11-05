//! A pipe of radius 1 along the helix of radius 10 about the z axis rising 5 a turn, from
//! (10, 0, 0) three right-handed turns, each end closed by a half ball of radius 1, as a ball
//! rolled along the helix would leave.
//!
//! cadrs has no sweep of a solid, so the ball's sweep is built as what it is: the disc square to
//! the helix swept along it (as in `sweep/helical-pipe`), and the ball at each end of the helix,
//! (10, 0, 0) and (10, 0, 15), whose half beyond the pipe's flat end closes it (the other half
//! lies inside the pipe). The kernel has no sphere primitive: each ball is a half disc revolved.
use crate::bench::{ball, p3, round, sketch, square_to, union_each};
use crate::shared::sweep::{helix, helix_at, sweep};
use crate::Refusal;
use cadrs_kernel::{BodyId, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    let o = p3(0.0, 0.0, 0.0);
    let (start, tangent) = helix_at(o, 10.0, 5.0, 0.0, 0.0);
    let section = sketch(square_to(start, tangent), vec![round((0.0, 0.0), 1.0)]);
    let pipe = sweep(k, &section, helix(o, 10.0, 5.0, 0.0, 3.0))?;
    let first = ball(k, p3(10.0, 0.0, 0.0), 1.0)?;
    let last = ball(k, p3(10.0, 0.0, 15.0), 1.0)?;
    union_each(k, pipe, &[first, last])
}
