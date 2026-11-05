//! A block (−10, −10, 0)–(10, 10, 10) united with the solid paraboloid x² + y² ≤ 8z, z ≤ 12.5, whose
//! vertex touches the middle of the block's bottom face from inside.
use crate::sketch::block;
use cad_kernel::boolean::{combined, Op};
use cad_kernel::build::{revolve, Profile, ProfileEdge};
use cad_kernel::nurbs::BSpline;
use cad_kernel::topo::Brep;
use std::f64::consts::TAU;
use std::sync::Arc;

const HALF_SIDE: f64 = 10.;
const HEIGHT: f64 = 10.;
const TOP: f64 = 12.5;
const RIM: f64 = 10.;

/// The parabola x² = 8z of the xz plane, from its vertex to x = 10, turned once about z.
fn paraboloid() -> Result<Brep, String> {
    // a profile has no parabola: its arc as the quadratic Bézier on the tangents at its ends, which
    // meet at (RIM/2, 0), halfway out under the rim
    let (vertex, rim, top) = ([0., 0., 0.], [RIM, 0., TOP], [0., 0., TOP]);
    let arc = BSpline::new(2, vec![0., 0., 0., 1., 1., 1.], vec![vertex, [RIM / 2., 0., 0.], rim])?;
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
    let stock = block([-HALF_SIDE, -HALF_SIDE, 0.], [HALF_SIDE, HALF_SIDE, HEIGHT])?;
    Ok(combined(&stock, &paraboloid()?, Op::Union, 0.)?)
}
