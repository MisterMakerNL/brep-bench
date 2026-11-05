//! The solid paraboloid x² + y² ≤ 10z, z ≤ 5, united with the ball of radius 5 about (0, 0, 5),
//! whose curvature matches the cup's at its vertex: they touch there only, to more than first order.
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{revolve, sphere, Profile, ProfileEdge};
use cad_kernel::nurbs::BSpline;
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;
use std::sync::Arc;

const TOP: f64 = 5.;
const RADIUS: f64 = 5.;

/// The parabola x² = 10z of the xz plane, from its vertex to the rim at z = 5, turned once about z.
fn cup() -> Result<Brep, String> {
    let rim_radius = (10. * TOP).sqrt();
    // a profile has no parabola: its arc as the quadratic Bézier on the tangents at its ends, which
    // meet halfway out under the rim
    let (vertex, rim, top) = ([0., 0., 0.], [rim_radius, 0., TOP], [0., 0., TOP]);
    let arc = BSpline::new(2, vec![0., 0., 0., 1., 1., 1.], vec![vertex, [rim_radius / 2., 0., 0.], rim])?;
    let section = Profile {
        origin: vertex,
        normal: [0., -1., 0.],
        loops: vec![vec![
            ProfileEdge::Spline(Arc::new(arc)),
            ProfileEdge::Line { a: rim, b: top },
            ProfileEdge::Line { a: top, b: vertex },
        ]],
        names: Vec::new(),
    };
    revolve(&section, [0., 0., 0.], [0., 0., 1.], TAU)
}

pub fn build() -> Result<Brep, String> {
    // the ball, its poles on the z axis: the lower one is the point of contact
    let ball = sphere([0., 0., RADIUS], [0., 0., 1.], RADIUS)?;
    Ok(combined(&cup()?, &ball, Op::Union, 0.)?)
}
