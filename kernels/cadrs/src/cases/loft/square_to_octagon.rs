//! A square-to-octagon transition: the square −20 ≤ x, y ≤ 20 at z = 0 lofted to the octagon at
//! z = 30 with corners (±6, ±14), (±14, ±6), each side of the square joined to the octagon's side
//! parallel to it by a flat trapezoid and each corner of the square to the octagon's cut side
//! facing it by a flat triangle.
//!
//! cadrs's solid Loft between the square and the octagon pairs their curves its own way (a
//! solid of 32843, not this one), and it takes no guides to pair a corner with a side, so the
//! transition is built as its surfacing would make it: each wall a surface Loft through curves,
//! a trapezoid from the square's side to the octagon's side parallel to it and a triangle from
//! the octagon's cut side to the corner of the square it faces, the floor and the top the Fills
//! of the square and the octagon, the ten sheets sewn into a solid.
use crate::bench::{line, p2, p3, xy};
use crate::shared::loft::{flat, sewn, surface_loft};
use crate::shared::sweep::on;
use crate::Refusal;
use cadrs_kernel::{BodyId, CurveSection, DefaultKernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    // the square's corners counter-clockwise from (20, −20), and the octagon's from (6, −14):
    // the corner k faces the octagon's cut side from corner 2k to 2k + 1, and the square's side
    // from corner k to k + 1 lies below the octagon's side from 2k + 1 to 2k + 2
    let square = [(20.0, -20.0), (20.0, 20.0), (-20.0, 20.0), (-20.0, -20.0)];
    let octagon = [
        (6.0, -14.0),
        (14.0, -6.0),
        (14.0, 6.0),
        (6.0, 14.0),
        (-6.0, 14.0),
        (-14.0, 6.0),
        (-14.0, -6.0),
        (-6.0, -14.0),
    ];
    let side = |z: f64, a: (f64, f64), b: (f64, f64)| {
        CurveSection::Wire(vec![on(xy(z), line(p2(a.0, a.1), p2(b.0, b.1)))])
    };
    let mut sheets = vec![flat(k, 0.0, &square)?, flat(k, 30.0, &octagon)?];
    for i in 0..4 {
        let (corner, next) = (square[i], square[(i + 1) % 4]);
        let (a, b, c) = (octagon[2 * i], octagon[2 * i + 1], octagon[(2 * i + 2) % 8]);
        let apex = CurveSection::Point(p3(corner.0, corner.1, 0.0));
        sheets.push(surface_loft(k, vec![side(30.0, a, b), apex], vec![])?);
        sheets.push(surface_loft(k, vec![side(0.0, corner, next), side(30.0, b, c)], vec![])?);
    }
    sewn(k, &sheets)
}
