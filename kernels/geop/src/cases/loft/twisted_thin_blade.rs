//! A thin blade from z = 0 to 60, its section at height z the rectangle 40 × 0.4 centred on the z
//! axis, turned right-handed 6° for every unit of height, a whole turn in all; its corners run
//! along four helices about the z axis rising 60 in a turn.
//!
//! The rectangle is sketched on the xy plane and again on a plane 60 up, where it has come back
//! to; the helices through three of its corners are drawn in 3-D sketches, and the rectangle is
//! lofted from one to the other guided by them. geop's loft takes at most three guides, which fix
//! any affine change of the sections — here a turn: the guides through three corners set the
//! section, and its fourth corner follows.
use crate::shared::paths::{at, loft, offset_plane, path3d, sketch3d};
use crate::shared::{Drawing, n, sketch, xy};
use crate::{Bench, Refusal};
use geop_core_math::primitives::FrameAxis;
use geop_ops_model::sketch3d::Sketch3d;

pub fn build(b: &mut Bench) -> Result<(), Refusal> {
    let mut foot = Drawing::new();
    foot.rectangle([-20.0, -0.2], [20.0, 0.2]);
    sketch(b, "foot", xy(), foot)?;
    let top = offset_plane(b, "top", FrameAxis::Z, 60.0)?;
    let mut head = Drawing::new();
    head.rectangle([-20.0, -0.2], [20.0, 0.2]);
    sketch(b, "head", top, head)?;

    let mut guides = Vec::new();
    for (id, corner) in [("helix_a", [20.0, -0.2]), ("helix_b", [20.0, 0.2]), ("helix_c", [-20.0, 0.2])] {
        let mut helix = Sketch3d::new();
        let start = helix.add_point(at([corner[0], corner[1], 0.0]));
        let base = helix.add_point(at([0.0, 0.0, 0.0]));
        let top = helix.add_point(at([0.0, 0.0, 60.0]));
        helix.add_helix(start, base, top, n(1.0), false)?;
        sketch3d(b, id, helix)?;
        guides.push(path3d(id));
    }
    loft(b, "blade", &["foot", "head"], guides)
}
