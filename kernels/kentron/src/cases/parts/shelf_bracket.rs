//! An L-shaped plate bracket, 10 thick: its outline in the xz plane extruded from y = −5 to y = 5.
//! The outline runs (x, z) from (0, 0) to (20, 0), up to (20, 92), round a quarter circle of
//! radius 8 centred at (28, 92) to (28, 100), on to (100, 100), up to (100, 120), back to (0, 120)
//! and down to (0, 0): a post 20 wide and 120 tall with an arm 20 deep along its top, the inside
//! corner rounded to radius 8. The outline is drawn with kentron's profile pen, the round an arc
//! tangent to the post turning right through 90°.
use crate::bench::v;
use crate::shared::parts::xz_plane;
use kentron::sketch::{self, Outcome, Step};
use kentron::sweep;
use kentron::topology::{Model, Shape};

pub fn build(m: &mut Model) -> kentron::Result<Shape> {
    use Step::{Arc, Start, To};
    let outline = [
        Start(0.0, 0.0),
        To(20.0, 0.0),
        To(20.0, 92.0),
        // centre on the right of the pen, heading up: (28, 92)
        Arc { radius: -8.0, angle: 90.0 },
        To(100.0, 100.0),
        To(100.0, 120.0),
        To(0.0, 120.0),
    ];
    let section = sketch::profile(m, xz_plane(-5.0), &outline, Outcome::Face)?;
    Ok(sweep::extrude(m, section, v(0.0, 10.0, 0.0))?.shape)
}
