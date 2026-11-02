//! A ball of radius 10 about the origin joined to a post, the cylinder of radius 4 on the z axis
//! from z = 0 to z = 15. The circle where the post leaves the ball, radius 4 at z = √84, is
//! rounded to radius 2, adding material: a torus tangent to the ball and the post.
use crate::bench::{edge_through, p};
use crate::Refusal;
use kentron::blend;
use kentron::boolean;
use kentron::primitives::{Cylinder, Sphere};
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> Result<Shape, Refusal> {
    let ball = Sphere::new(10.0).build(m)?;
    let post = Cylinder::new(4.0, 15.0).build(m)?;
    let part = boolean::union(m, ball, post)?;
    let root = edge_through(m, part, p(0.0, 4.0, 84f64.sqrt()))?;
    Ok(blend::fillet(m, part, &[root], 2.0)?)
}
