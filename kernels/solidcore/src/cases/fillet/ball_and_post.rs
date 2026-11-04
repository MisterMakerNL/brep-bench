//! A ball of radius 10 about the origin and a post, the cylinder of radius 4 on the z axis from
//! z = 0 to z = 15, united; the circle where the post leaves the ball, radius 4 at z = √84,
//! rounded to radius 2, adding material.
use crate::bench::{edge_through, fuse, reloaded};
use crate::Refusal;
use solidcore_ops::{blend, make_cylinder, make_sphere, Blend, Body, Corners};

pub fn build() -> Result<Body, Refusal> {
    let ball = make_sphere([0.0, 0.0, 0.0], 10.0)?;
    let post = make_cylinder([0.0, 0.0, 0.0], 4.0, 15.0)?;
    let (body, order) = reloaded(&fuse(ball, post)?)?;
    let root = edge_through(&body, &order, [0.0, 4.0, 84f64.sqrt()])?;
    blend(&body, Blend::Round, &[root], &[2.0], &[], Corners::Filled)?;
    Ok(body)
}
