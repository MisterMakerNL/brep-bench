//! An L-shaped plate: the polygon through (0, 0), (20, 0), (20, 5), (5, 5), (5, 15) and (0, 15) in
//! the xy plane, extruded 8 along +z. The outline is drawn with kentron's profile pen.
use crate::bench::v;
use kentron::math::Frame;
use kentron::sketch::{self, Outcome, Step};
use kentron::sweep;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    use Step::{Start, To};
    let outline = [Start(0.0, 0.0), To(20.0, 0.0), To(20.0, 5.0), To(5.0, 5.0), To(5.0, 15.0), To(0.0, 15.0)];
    let face = sketch::profile(m, Frame::WORLD, &outline, Outcome::Face)?;
    Ok(sweep::extrude(m, face, v(0.0, 0.0, 8.0))?.shape)
}
