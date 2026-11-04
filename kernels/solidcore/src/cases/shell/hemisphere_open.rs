//! The lower half of the ball of radius 5 centred on the origin, shelled inward to a wall 1 thick,
//! open at its flat face, z = 0: a bowl, the half ball less the half ball of radius 4. The half
//! ball is the ball less a box above z = 0.
use crate::bench::{cut, face_through, order};
use crate::Refusal;
use solidcore_ops::{make_box, make_sphere, shell, Body};

pub fn build() -> Result<Body, Refusal> {
    let ball = make_sphere([0.0, 0.0, 0.0], 5.0)?;
    let above = make_box([-6.0, -6.0, 0.0], [12.0, 12.0, 6.0])?;
    let half = cut(ball, above)?;
    let flat = face_through(&half, [0.0, 0.0, 0.0])?;
    shell(&half, &order(&half)?, &[flat], -1.0)?;
    Ok(half)
}
