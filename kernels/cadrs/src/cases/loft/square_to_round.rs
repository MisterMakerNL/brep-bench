//! A square-to-round transition: the square −20 ≤ x, y ≤ 20 at z = 0 lofted to the circle of
//! radius 15 about the z axis at z = 30, each side of the square joined by a flat triangle to the
//! point of the circle nearest it and each corner to the quarter circle between by a piece of an
//! oblique cone, its apex the corner.
//!
//! cadrs's solid Loft between the square and the circle pairs their curves its own way (a solid
//! of 33872, not this one), and it takes no guides to pair a side with a point or a corner with
//! an arc, so the transition is built as its surfacing would make it: each wall a surface Loft
//! through curves, a triangle from the square's side to the point of the circle nearest it and a
//! cone from the quarter of the circle to the corner of the square between, the floor and the
//! top the Fills of the square and of the circle's four quarters, the ten sheets sewn into a solid.
use crate::bench::{arc, line, p2, p3, xy};
use crate::shared::loft::{flat, sewn, surface_loft};
use crate::shared::sweep::{first, on};
use crate::Refusal;
use cadrs_kernel::{BodyId, CurveSection, DefaultKernel, FillCurve, FillSpec, Kernel};

pub fn build(k: &mut DefaultKernel) -> Result<BodyId, Refusal> {
    // the square's corners counter-clockwise from (20, −20), and the circle's points nearest
    // its sides from (0, −15): the side from corner k − 1 to k faces point k, and corner k lies
    // between points k and k + 1
    let square = [(20.0, -20.0), (20.0, 20.0), (-20.0, 20.0), (-20.0, -20.0)];
    let points = [(0.0, -15.0), (15.0, 0.0), (0.0, 15.0), (-15.0, 0.0)];
    // the circle in its four quarters, each the rim of one cone
    let quarter = |i: usize| arc(p2(0.0, 0.0), 15.0, -90.0 + 90.0 * i as f64, 90.0);
    let rim = (0..4).map(|i| FillCurve::Sketch { plane: xy(30.0), curve: quarter(i) }).collect();
    let top = FillSpec { curves: rim, source: 1 };
    let mut sheets = vec![flat(k, 0.0, &square)?, first(k.fill(&top)?)?];
    for i in 0..4 {
        let (before, corner) = (square[(i + 3) % 4], square[i]);
        let side = on(xy(0.0), line(p2(before.0, before.1), p2(corner.0, corner.1)));
        let point = CurveSection::Point(p3(points[i].0, points[i].1, 30.0));
        sheets.push(surface_loft(k, vec![CurveSection::Wire(vec![side]), point], vec![])?);
        let apex = CurveSection::Point(p3(corner.0, corner.1, 0.0));
        let rim = CurveSection::Wire(vec![on(xy(30.0), quarter(i))]);
        sheets.push(surface_loft(k, vec![rim, apex], vec![])?);
    }
    sewn(k, &sheets)
}
