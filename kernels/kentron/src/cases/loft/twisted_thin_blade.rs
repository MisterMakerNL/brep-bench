//! A thin blade twisted through a whole turn, z from 0 to 60: its section at height z the
//! rectangle 40 × 0.4 centred on the z axis, turned right-handed by 6° per unit of height, its
//! corners on four helices of radius √(20² + 0.2²) rising 60 a turn; each wall swept by a side
//! of the rectangle, a straight horizontal segment between neighbouring helices.
//!
//! A sweep of the rectangle up the axis (`pipe_through_with`, OCCT's MakePipeShell), the helix
//! through the corner (20, 0.2, 0) its guide: at each height the rectangle turns so that its
//! corner keeps pointing at where the plane at that height meets the helix. The helix is
//! kentron's B-spline approximation of it, within 1e-7.
use crate::bench::p;
use crate::shared::loft::polygon;
use kentron::helix::{self, HelixOptions, HelixPart};
use kentron::make;
use kentron::math::{Direction, Frame};
use kentron::sweep::{self, Guide, PipeOptions};
use kentron::topology::{Model, Shape};

const HEIGHT: f64 = 60.0;
const HALF_LENGTH: f64 = 20.0;
const HALF_WIDTH: f64 = 0.2;

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    let (a, b) = (HALF_LENGTH, HALF_WIDTH);
    let foot = polygon(m, &[p(a, b, 0.0), p(-a, b, 0.0), p(-a, -b, 0.0), p(a, -b, 0.0)])?;
    let axis = make::edge_between(m, p(0.0, 0.0, 0.0), p(0.0, 0.0, HEIGHT))?;
    // the helix through the corner (a, b, 0): a whole turn in the height, right-handed
    let diameter = 2.0 * a.hypot(b);
    let start = Frame::new(p(0.0, 0.0, 0.0), Direction::Z, Direction::new(a, b, 0.0));
    let part = HelixPart { height: HEIGHT, pitch: HEIGHT, end_diameter: diameter };
    let options = HelixOptions { tolerance: 1e-7, ..HelixOptions::default() };
    let guide = helix::helix(m, &start, diameter, &[part], &options)?.wire;
    let guide = Guide::Wire { wire: guide, curvilinear: false };
    Ok(sweep::pipe_through_with(m, &[(foot, PipeOptions::default())], axis, true, Some(guide))?.shape)
}
